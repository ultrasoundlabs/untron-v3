import { readFileSync } from "node:fs";

export function loadDotEnvLocal(args = {}) {
  const { cwd = process.cwd(), filename = ".env.local" } = args;
  const path = new URL(`${cwd.replace(/\/?$/, "/")}${filename}`, "file:///").pathname;

  try {
    const raw = readFileSync(path, "utf8");
    for (const line of raw.split("\n")) {
      const trimmed = line.trim();
      if (!trimmed || trimmed.startsWith("#")) continue;
      const eq = trimmed.indexOf("=");
      if (eq === -1) continue;
      const key = trimmed.slice(0, eq).trim();
      let value = trimmed.slice(eq + 1).trim();
      if (
        (value.startsWith('"') && value.endsWith('"')) ||
        (value.startsWith("'") && value.endsWith("'"))
      ) {
        value = value.slice(1, -1);
      }
      if (key && process.env[key] == null) process.env[key] = value;
    }
  } catch {
    // ignore missing .env.local
  }
}

export async function graphqlRequest(args) {
  const { url, query, variables } = args;
  const res = await fetch(url, {
    method: "POST",
    headers: { "content-type": "application/json" },
    body: JSON.stringify({ query, variables }),
  });

  const json = await res.json().catch(() => null);
  if (!res.ok) {
    const details = json ?? { message: `HTTP ${res.status}` };
    throw new Error(`[graphql] request failed: ${JSON.stringify(details)}`);
  }
  if (!json || typeof json !== "object") {
    throw new Error(`[graphql] invalid JSON response: ${String(json)}`);
  }
  if (json.errors && Array.isArray(json.errors) && json.errors.length > 0) {
    throw new Error(`[graphql] errors: ${JSON.stringify(json.errors)}`);
  }
  return json.data;
}
