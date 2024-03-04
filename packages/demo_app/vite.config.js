import { URL, fileURLToPath } from "url";

import { defineConfig } from "vite";
import dotenv from "dotenv";
import environment from "vite-plugin-environment";
import react from "@vitejs/plugin-react";

dotenv.config({ path: "../../.env" });

process.env.II_URL =
  process.env.DFX_NETWORK === "local"
    ? `http://${process.env.INTERNET_IDENTITY_CANISTER_ID}.localhost:4943/`
    : `https://identity.ic0.app`;

export default defineConfig({
  build: {
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
  },
  plugins: [
    react(),
    environment("all", { prefix: "CANISTER_" }),
    environment("all", { prefix: "DFX_" }),
    environment(["II_URL"]),
  ],
  resolve: {
    alias: [
      {
        find: "declarations",
        replacement: fileURLToPath(new URL("../declarations", import.meta.url)),
      },
    ],
  },
});
