import { defineConfig } from "vite";
import vue from "@vitejs/plugin-vue";

// @ts-expect-error process 是 NodeJS 全局变量
const host = process.env.TAURI_DEV_HOST;

// https://vite.dev/config/
export default defineConfig(async () => ({
  plugins: [
    vue({
      template: {
        compilerOptions: {
          // 告诉 Vue 所有 mdui- 开头的标签都是自定义元素
          isCustomElement: (tag) => tag.startsWith('mdui-')
        }
      }
    })
  ],

  // 专为 Tauri 开发定制的 Vite 选项，仅在 `tauri dev` 或 `tauri build` 时应用
  //
  // 1. 防止 Vite 遮挡 Rust 错误
  clearScreen: false,
  // 2. Tauri 需要固定端口，如果端口不可用则报错
  server: {
    port: 1420,
    strictPort: true,
    host: host || "127.0.0.1",
    hmr: host
      ? {
        protocol: "ws",
        host,
        port: 1421,
      }
      : undefined,
    watch: {
      // 3. 告诉 Vite 忽略监听 `src-tauri`
      ignored: ["**/src-tauri/**"],
    },
  },
}));
