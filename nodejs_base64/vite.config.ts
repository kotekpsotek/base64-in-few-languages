import { defineConfig } from "vitest/config";

export default defineConfig({
    test: {
        include: ["./src/index.test.ts"],
        // exclude: ["*"]
    }
})