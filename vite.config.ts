// SilenceEvolution
// Copyright (C) 2026 Oscar Alvarez Gonzalez

import { defineConfig } from "vite";

import { resolve } from "path";

import solid from "vite-plugin-solid";
import tailwindcss from "@tailwindcss/vite";

import { imagetools } from "vite-imagetools";

export default defineConfig(({ mode }) => {
    const isProd = mode === "production";

    return {
        base: "/",
        root: "./frontend",
        build: {
            outDir: "../target/frontend",
            assetsDir: "internal_assets",
            minify: isProd,
            cssMinify: isProd,
            cssCodeSplit: true,
            sourcemap: "hidden",
            emptyOutDir: true,
            rollupOptions: {
                input: {
                    admin: resolve(__dirname, "./frontend/admin/index.html"),
                    auth: resolve(__dirname, "./frontend/auth/index.html"),
                },
            },
        },
        plugins: [
            solid({ hot: true }),
            tailwindcss({ optimize: true }),
            imagetools(),
        ],
    };
});
