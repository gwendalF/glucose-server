import { defineConfig, presetUno } from "unocss";

export default defineConfig({
  presets: [presetUno()],
  cli: {
    entry: {
      patterns: ["templates/**/*.html"],
      outFile: "./assets/style.css",
    },
  },
});
