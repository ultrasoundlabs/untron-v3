import { buildMergedOpenapi3 } from "./convert";

async function readStdin(): Promise<string> {
  return await new Promise((resolve, reject) => {
    let data = "";
    process.stdin.setEncoding("utf8");
    process.stdin.on("data", (chunk) => (data += chunk));
    process.stdin.on("end", () => resolve(data));
    process.stdin.on("error", reject);
  });
}

async function main() {
  const args = process.argv.slice(2);
  const includeRealtor = args.includes("--include-realtor");

  const upstreamText = await readStdin();
  const upstreamSwagger2 = JSON.parse(upstreamText);

  let realtorOpenapi3: any = null;
  if (includeRealtor) {
    const url = process.env.REALTOR_OPENAPI_URL;
    if (!url) throw new Error("REALTOR_OPENAPI_URL must be set with --include-realtor");
    const res = await fetch(url, { headers: { Accept: "application/json" } });
    if (res.ok) realtorOpenapi3 = await res.json();
  }

  const spec = await buildMergedOpenapi3({ upstreamSwagger2, realtorOpenapi3 });
  process.stdout.write(JSON.stringify(spec, null, 2) + "\n");
}

main().catch((e) => {
  // eslint-disable-next-line no-console
  console.error(String(e?.stack ?? e));
  process.exit(1);
});
