import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'


const wasmContentTypePlugin = {
  name: "wasm-content-type-plugin",
  configureServer(server) {
    server.middlewares.use((req, res, next) => {
      if (req.url.endsWith(".wasm")) {
        res.setHeader("Content-Type", "application/wasm");
      }
      next();
    });
  },
};
// https://vitejs.dev/config/
export default defineConfig({
  plugins: [react(), wasmContentTypePlugin],
})
