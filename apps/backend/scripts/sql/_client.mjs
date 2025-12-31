export async function sqlDbRequest(args) {
  const { url, sql, params = [] } = args;
  // Ponder's `/sql/db` expects a `superjson.stringify()` payload.
  // For plain JSON-safe values (string/number/null/array/object), this is equivalent to:
  // `JSON.stringify({ json: value })`.
  const queryString = JSON.stringify({ json: { sql, params } });

  const fullUrl = new URL(url);
  fullUrl.searchParams.set("sql", queryString);

  const res = await fetch(fullUrl.toString(), { method: "GET" });
  const text = await res.text();

  if (!res.ok) {
    throw new Error(`[sql] request failed: HTTP ${res.status} ${text}`);
  }

  try {
    return JSON.parse(text);
  } catch {
    throw new Error(`[sql] invalid JSON response: ${text}`);
  }
}
