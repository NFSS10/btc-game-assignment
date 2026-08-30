import js from "@eslint/js";
import globals from "globals";
import reactHooks from "eslint-plugin-react-hooks";
import reactRefresh from "eslint-plugin-react-refresh";
import tseslint from "typescript-eslint";
import { defineConfig, globalIgnores } from "eslint/config";

const isProd = process.env.NODE_ENV === "production";

export default defineConfig([
    {
        files: ["**/*.{ts,tsx}"],
        extends: [
            js.configs.recommended,
            tseslint.configs.recommended,
            reactHooks.configs.flat.recommended,
            reactRefresh.configs.vite
        ],
        languageOptions: {
            globals: globals.browser
        },
        rules: {
            "space-before-function-paren": ["error", { anonymous: "never", named: "never", asyncArrow: "always" }],
            "no-console": isProd ? ["warn", { allow: ["warn", "error", "info"] }] : "off",
            "no-debugger": isProd ? "warn" : "off",
            "linebreak-style": ["error", "unix"],
            "@typescript-eslint/no-unused-vars": ["warn", { argsIgnorePattern: "^_", varsIgnorePattern: "^_" }],
            "@typescript-eslint/consistent-type-imports": "error"
        }
    },
    globalIgnores(["dist"])
]);
