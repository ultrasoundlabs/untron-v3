import { defineConfig } from "@wagmi/cli";
import { foundry } from "@wagmi/cli/plugins";

// Determine artifacts directory based on FOUNDRY_PROFILE env variable
const profile = process.env.FOUNDRY_PROFILE ?? "dev";
const artifactsPath = `out/${profile}`;

export default defineConfig({
  out: "packages/contracts/abi/generated.ts",
  plugins: [
    foundry({
      project: "packages/contracts", // <— your Foundry project root
      artifacts: artifactsPath,
      include: ["**/*.json"], // include all artifacts
      exclude: [
        "**/*.dbg.json",
        "build-info/**",
        // Exclude forge-std and test artifacts
        "**/Base.sol/**",
        "**/console*.sol/**",
        "**/Script.sol/**",
        "**/Test.sol/**",
        "**/Std*.sol/**",
        "**/Vm.sol/**",
        "**/IMulticall3.sol/**",
        "**/*.t.sol/**", // exclude test contracts
        "**/*.s.sol/**", // exclude script contracts
      ],
      // forge: { build: true } // default; Wagmi can run forge build for you
    }),
  ],
});
