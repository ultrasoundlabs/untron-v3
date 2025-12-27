export function getRows(result: unknown): unknown[] {
  if (Array.isArray(result)) return result;
  if (result && typeof result === "object" && "rows" in result) {
    const rows = (result as { readonly rows?: unknown }).rows;
    if (Array.isArray(rows)) return rows;
  }
  return [];
}
