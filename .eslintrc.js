/* eslint-disable */

module.exports = {
    // "env": [
    //   'plugin:vue/vue3-essential',
    //   'eslint:recommended',
    //   '@vue/typescript'
    // ],

    "extends": [
        "plugin:vue/vue3-essential",
        "eslint:recommended",
        '@vue/typescript'
    ],

    "globals": {
        "Atomics": "readonly",
        "SharedArrayBuffer": "readonly"
    },

    "rules": {
        "quotes": ["warn", "double"],
        "indent": ["error", 4],
        // "semi": ["warn", "always"],
        "comma-dangle": ["warn", "never"],
        "eol-last": ["warn", "always"],
    },

    "ignorePatterns": [
        "node_modules/**/*",
        "src/**/*",
        "mona_wasm/**/*",
        "loaders/**/*",
        "sub/**/*",
        // "**/*"
    ],

    parserOptions: {
      parser: '@typescript-eslint/parser',
      ecmaVersion: 2020,
      sourceType: 'module'
    }
};
