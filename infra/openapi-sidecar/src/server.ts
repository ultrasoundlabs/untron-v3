import { serve } from "@hono/node-server";
import { Hono } from "hono";
import { buildMergedOpenapi3 } from "./convert";
import { DEFAULT_OPENAPI_INFO } from "./branding";

const UPSTREAM_OPENAPI_URL = (process.env.UPSTREAM_OPENAPI_URL ?? "http://postgrest:3000/").trim();
const REALTOR_OPENAPI_URL = (process.env.REALTOR_OPENAPI_URL ?? "").trim();
const CACHE_SECONDS = Number.parseInt(process.env.CACHE_SECONDS ?? "30", 10);
const OPENAPI_TITLE = (process.env.OPENAPI_TITLE ?? "").trim();
const OPENAPI_DESCRIPTION = (process.env.OPENAPI_DESCRIPTION ?? "").trim();
const OPENAPI_VERSION = (process.env.OPENAPI_VERSION ?? "").trim();

type Cache = { body: string | null; atMs: number };
const cache: Cache = { body: null, atMs: 0 };

async function fetchJson(url: string, opts?: { accept?: string; timeoutMs?: number }) {
  const ctrl = new AbortController();
  const timeout = setTimeout(() => ctrl.abort(), opts?.timeoutMs ?? 10_000);
  try {
    const res = await fetch(url, {
      headers: { Accept: opts?.accept ?? "application/json" },
      signal: ctrl.signal,
    });
    if (!res.ok) {
      const text = await res.text().catch(() => "");
      throw new Error(`GET ${url} -> ${res.status} ${res.statusText}: ${text}`);
    }
    return await res.json();
  } finally {
    clearTimeout(timeout);
  }
}

async function buildOpenapiJson(): Promise<string> {
  const upstreamSwagger2 = await fetchJson(UPSTREAM_OPENAPI_URL, {
    accept: "application/openapi+json",
    timeoutMs: 10_000,
  });

  let realtorOpenapi3: any = null;
  if (REALTOR_OPENAPI_URL) {
    try {
      realtorOpenapi3 = await fetchJson(REALTOR_OPENAPI_URL, { timeoutMs: 500 });
    } catch {
      realtorOpenapi3 = null;
    }
  }

  const spec = await buildMergedOpenapi3({ upstreamSwagger2, realtorOpenapi3 });

  const info: any = typeof spec.info === "object" && spec.info ? spec.info : {};
  info.title = OPENAPI_TITLE || DEFAULT_OPENAPI_INFO.title;
  info.description = OPENAPI_DESCRIPTION || DEFAULT_OPENAPI_INFO.description;
  if (OPENAPI_VERSION) info.version = OPENAPI_VERSION;
  spec.info = info;

  return JSON.stringify(spec);
}

const app = new Hono();

app.get("/healthz", (c) => c.json({ ok: true }));

app.get("/openapi.json", async (c) => {
  const now = Date.now();
  const fresh = cache.body !== null && now - cache.atMs <= CACHE_SECONDS * 1000;
  if (!fresh) {
    cache.body = await buildOpenapiJson();
    cache.atMs = now;
  }
  return c.body(cache.body!, 200, { "content-type": "application/json" });
});

serve(
  {
    fetch: app.fetch,
    hostname: "0.0.0.0",
    port: 5000,
  },
  (info) => {
    // eslint-disable-next-line no-console
    console.log(`openapi-sidecar listening on http://${info.address}:${info.port}`);
  }
);
