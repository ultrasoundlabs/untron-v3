import { mkdir, rm } from "node:fs/promises";
import { execFile } from "node:child_process";
import { promisify } from "node:util";
import path from "node:path";

const execFileAsync = promisify(execFile);

const root = path.resolve(import.meta.dirname);
const dist = path.join(root, "dist");

await rm(dist, { recursive: true, force: true });
await mkdir(dist, { recursive: true });

const esbuildBin = path.resolve(root, "../../node_modules/.bin/esbuild");
const commonArgs = [
  "--bundle",
  "--platform=node",
  "--target=node20",
  "--format=cjs",
  "--log-level=info",
];

await execFileAsync(esbuildBin, [
  path.join(root, "src/server.ts"),
  ...commonArgs,
  `--outfile=${path.join(dist, "server.cjs")}`,
]);

await execFileAsync(esbuildBin, [
  path.join(root, "src/cli.ts"),
  ...commonArgs,
  `--outfile=${path.join(dist, "cli.cjs")}`,
]);
