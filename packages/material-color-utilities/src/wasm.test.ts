import * as typescript from "@material/material-color-utilities";
import * as rust from "./wasm";

import { test } from "vitest";

test("wasm", () => {
  for (let i = 0; i < 10; i++) {
    console.time("rust");
    const result = rust.material_dynamic_colors({ isDark: false });
    console.timeEnd("rust");
  }
  // console.log(result);

  for (let i = 0; i < 10; i++) {
    console.time("typescript");
    const colors = new typescript.MaterialDynamicColors();
    const scheme = new typescript.DynamicScheme({
      isDark: false,
      sourceColorHct: typescript.Hct.fromInt(0xffff0000),
      variant: typescript.Variant.TONAL_SPOT,
    });
    const result: Record<string, string> = {};
    for (const color of colors.allDynamicColors) {
      result[color.name] = typescript.hexFromArgb(color.getArgb(scheme));
    }
    // console.log(result);
    console.timeEnd("typescript");
  }

  // wasm.initSync(wasmModule);
});
