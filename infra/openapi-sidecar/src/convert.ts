import type { OpenAPIObject } from "./openapi-types";

type Swagger2 = Record<string, unknown>;

type Swagger2OpenapiModule = {
  convertObj: (
    swagger: Swagger2,
    options: Record<string, unknown>
  ) => Promise<{ openapi: OpenAPIObject }>;
};

async function loadSwagger2Openapi(): Promise<Swagger2OpenapiModule> {
  const mod: any = await import("swagger2openapi");
  return (mod?.default ?? mod) as Swagger2OpenapiModule;
}

function deepWalk(value: unknown, visit: (node: any) => void) {
  if (Array.isArray(value)) {
    for (const v of value) deepWalk(v, visit);
    return;
  }
  if (value && typeof value === "object") {
    visit(value);
    for (const v of Object.values(value as Record<string, unknown>)) deepWalk(v, visit);
  }
}

function keepGetOnly(spec: OpenAPIObject): OpenAPIObject {
  const out: OpenAPIObject = structuredClone(spec);
  const paths = out.paths ?? {};
  for (const [path, ops] of Object.entries(paths)) {
    if (!ops || typeof ops !== "object") continue;
    const getOp = (ops as any).get;
    if (!getOp || typeof getOp !== "object") {
      delete (paths as any)[path];
      continue;
    }
    (paths as any)[path] = { get: getOp };
  }
  out.paths = paths;
  return out;
}

function setGatewayServers(spec: OpenAPIObject): OpenAPIObject {
  const out: OpenAPIObject = structuredClone(spec);
  out.servers = [{ url: "/" }];
  return out;
}

function progenitorFriendly(spec: OpenAPIObject): OpenAPIObject {
  const out: OpenAPIObject = structuredClone(spec);

  deepWalk(out, (node) => {
    if (!node || typeof node !== "object") return;

    // PostgREST sometimes emits invalid `format`s for primitives.
    if (node.type === "boolean" && typeof node.format === "string") {
      delete node.format;
    }

    // If a schema looks like Postgres json/jsonb, coerce to a generic object.
    if (!node.type && (node.format === "jsonb" || node.format === "json")) {
      const desc = typeof node.description === "string" ? node.description : undefined;
      for (const k of Object.keys(node)) delete node[k];
      node.type = "object";
      node.additionalProperties = {};
      if (desc) node.description = desc;
      return;
    }

    // Ensure schemas have a `type` if they look like schema objects.
    if (!node.type) {
      if (node.properties) node.type = "object";
      else if (node.items) node.type = "array";
      else if (node.enum) node.type = "string";
    }
  });

  // Drop non-200 responses without a body to avoid Progenitor's multi-response assertion.
  for (const ops of Object.values(out.paths ?? {})) {
    if (!ops || typeof ops !== "object") continue;
    for (const op of Object.values(ops as any)) {
      if (!op || typeof op !== "object") continue;
      const responses = (op as any).responses;
      if (!responses || typeof responses !== "object") continue;
      for (const [status, resp] of Object.entries(responses)) {
        if (status === "200") continue;
        if (!resp || typeof resp !== "object") continue;
        const content = (resp as any).content;
        if (!content || typeof content !== "object" || !("application/json" in content)) {
          delete (responses as any)[status];
        }
      }
    }
  }

  // Normalize Postgres NUMERIC -> stable schema, then rely on Progenitor replacement.
  out.components ??= {};
  out.components.schemas ??= {};
  (out.components.schemas as any).PgNumeric ??= {
    type: "number",
    format: "numeric",
    description: "Postgres NUMERIC encoded as a JSON number.",
  };

  deepWalk(out, (node) => {
    if (!node || typeof node !== "object") return;
    if (node.type === "number" && node.format === "numeric") {
      const desc = typeof node.description === "string" ? node.description : undefined;
      for (const k of Object.keys(node)) delete node[k];
      node.allOf = [{ $ref: "#/components/schemas/PgNumeric" }];
      if (desc) node.description = desc;
    }
  });

  return out;
}

function mergeNamed(base: Record<string, any>, extra: Record<string, any>, prefix: string) {
  for (const [name, obj] of Object.entries(extra)) {
    if (!(name in base)) {
      base[name] = obj;
      continue;
    }
    if (JSON.stringify(base[name]) === JSON.stringify(obj)) continue;
    base[`${prefix}${name}`] = obj;
  }
}

export function mergeOpenapi3(base: OpenAPIObject, extra: OpenAPIObject): OpenAPIObject {
  return mergeOpenapi3WithPrefix(base, extra, "realtor_");
}

export function mergeOpenapi3WithPrefix(
  base: OpenAPIObject,
  extra: OpenAPIObject,
  collisionPrefix: string
): OpenAPIObject {
  const out: OpenAPIObject = structuredClone(base);

  out.paths ??= {};
  for (const [path, ops] of Object.entries(extra.paths ?? {})) {
    if (!(path in out.paths!)) {
      (out.paths as any)[path] = ops;
      continue;
    }
    if (typeof (out.paths as any)[path] === "object" && typeof ops === "object") {
      Object.assign((out.paths as any)[path], ops);
    }
  }

  if (Array.isArray(out.tags) && Array.isArray(extra.tags)) {
    const existing = new Set(out.tags.map((t: any) => t?.name).filter(Boolean));
    for (const t of extra.tags as any[]) {
      if (t?.name && !existing.has(t.name)) out.tags.push(t);
    }
  }

  out.components ??= {};
  const outComponents: any = out.components;
  const extraComponents: any = extra.components ?? {};

  for (const kind of ["schemas", "parameters", "responses", "requestBodies"] as const) {
    const src = extraComponents[kind];
    if (!src || typeof src !== "object") continue;
    outComponents[kind] ??= {};
    mergeNamed(outComponents[kind], src, collisionPrefix);
  }

  return out;
}

export async function convertSwagger2ToOpenapi3(swagger2: Swagger2): Promise<OpenAPIObject> {
  const converter = await loadSwagger2Openapi();
  const { openapi } = await converter.convertObj(swagger2, {
    patch: true,
    warnOnly: true,
    resolveInternal: true,
    targetVersion: "3.0.3",
  });
  return openapi;
}

export async function buildMergedOpenapi3(params: {
  upstreamSwagger2: Swagger2;
  realtorOpenapi3?: OpenAPIObject | null;
}): Promise<OpenAPIObject> {
  const realtor = params.realtorOpenapi3;
  let indexer = await convertSwagger2ToOpenapi3(params.upstreamSwagger2);
  indexer = keepGetOnly(indexer);
  indexer = setGatewayServers(indexer);
  indexer = progenitorFriendly(indexer);

  let merged: OpenAPIObject = indexer;

  if (realtor && typeof realtor === "object" && typeof realtor.openapi === "string") {
    if (realtor.openapi.startsWith("3.")) {
      merged = mergeOpenapi3WithPrefix(realtor, indexer, "indexer_");
      merged = setGatewayServers(merged);

      const indexerInfo: any = indexer.info;
      const mergedInfo: any = typeof merged.info === "object" && merged.info ? merged.info : {};
      if (typeof indexerInfo?.version === "string" && indexerInfo.version.trim()) {
        mergedInfo.version = indexerInfo.version;
      }
      merged.info = mergedInfo;

      if (realtor.openapi.startsWith("3.1")) {
        merged.openapi = "3.1.0";
      }
    }
  }

  return merged;
}
