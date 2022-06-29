import { defineConfig } from 'vite';
import { svelte } from '@sveltejs/vite-plugin-svelte';
import { svelteSVG } from "rollup-plugin-svelte-svg";

// https://vitejs.dev/config/
export default defineConfig({
    plugins: [
        svelte(),
        svelteSVG({
            svgo: {},
            enforce: "pre",
        }),
    ],
    optimizeDeps: { exclude: ["svelte-navigator"] },
    build: {
        outDir: '../static'
    },
    server: {
        watch: {
            usePolling: true
        }
    }
})
