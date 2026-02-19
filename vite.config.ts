// SilenceEvolution
// Copyright (C) 2026 Oscar Alvarez Gonzalez

import { defineConfig } from 'vite'

import solid from 'vite-plugin-solid'
import tailwindcss from '@tailwindcss/vite'

import { imagetools } from "vite-imagetools";

export default defineConfig(({ mode }) => {
    const isProd = mode === 'production'

    return {
        base: '',
        root: './frontend',
        build: {
            outDir: '../target/frontend',
            minify: isProd,
            cssMinify: isProd,
            cssCodeSplit: true,
            sourcemap: 'hidden',
            emptyOutDir: true,

        },
        plugins: [solid({ hot: true }), tailwindcss({ optimize: true }), imagetools()],
    }
})
