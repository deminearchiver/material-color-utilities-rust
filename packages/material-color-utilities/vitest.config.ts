import { defineConfig } from "vitest/config";
import wasm from "vite-plugin-wasm";

export default defineConfig({
  build: {
    target: "esnext",
  },
  plugins: [wasm()],
});
