import { defineConfig } from "tsdown";
import { wasm } from "@rollup/plugin-wasm";

export default defineConfig({
  entry: ["./src/index.ts"],
  platform: "neutral",
  plugins: [
    wasm({
      sync: ["./generated/index_bg.wasm"],
    }),
  ],
});
