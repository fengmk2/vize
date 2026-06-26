import { defineConfig } from "vite-plus";

export default defineConfig({
  lint: {
    plugins: ["vue"],
    jsPlugins: [
      "../../npm/lint-oxlint/dist/index.mjs",
      {
        name: "vite-plus",
        specifier: "vite-plus/oxlint-plugin",
      },
    ],
    settings: {
      vize: {
        locale: "en",
        helpLevel: "none",
      },
    },
    rules: {
      "no-console": "warn",
      "vite-plus/prefer-vite-plus-imports": "error",
    },
    options: {
      typeAware: true,
      typeCheck: true,
    },
  },
});
