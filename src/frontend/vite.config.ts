/// <reference types="vitest" />
import react from "@vitejs/plugin-react";
import { defineConfig } from "vite";
import environment from "vite-plugin-environment";
import dotenv from "dotenv";
import tailwindcss from "@tailwindcss/vite";
import { fileURLToPath, URL } from "url";

dotenv.config({ path: "../../.env" });

export default defineConfig({
  root: __dirname,
  build: {
    outDir: "dist/",
    emptyOutDir: true,
  },
  optimizeDeps: {
    esbuildOptions: {
      define: {
        global: "globalThis",
      },
    },
  },
  server: {
    proxy: {
      "/api": {
        target: "http://127.0.0.1:4943",
        changeOrigin: true,
      },
    },
    allowedHosts: [],
  },
  plugins: [
    react(),
    tailwindcss(),
    environment("all", { prefix: "CANISTER_" }),
    environment("all", { prefix: "DFX_" }),
  ],
  resolve: {
    alias: [
      {
        find: "declarations",
        replacement: fileURLToPath(new URL("../declarations", import.meta.url)),
      },
      // Add alias for Identity Kit to fix test resolution
      {
        find: "@nfid/identitykit/react",
        replacement: fileURLToPath(
          new URL(
            "../../node_modules/@nfid/identitykit/dist/libs/react/index.esm.js",
            import.meta.url,
          ),
        ),
      },
    ],
    dedupe: ["@dfinity/agent"],
    conditions: ["import", "module", "browser", "default"],
  },
  test: {
    environment: "jsdom",
    setupFiles: "frontend-test-setup.ts",
    globals: true,
  },
});
