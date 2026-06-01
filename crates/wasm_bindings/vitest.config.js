import { defineConfig } from 'vitest/config';
import wasm from 'vite-plugin-wasm';
import path from 'path';

export default defineConfig({
    plugins: [wasm()],
    test: {
        include: ['tests/**/*.test.{js,ts}'],
    },
    resolve: {
        alias: {
            '@': path.resolve(__dirname),
            bencode_wasm: path.resolve(__dirname, 'pkg/bencode_wasm.js'),
        },
    },
});
