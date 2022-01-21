/* eslint-disable */

module.exports = {
    "env": {
        "browser": true,
        "es6": true
    },
    "extends": [
        "plugin:vue/essential",
        "standard"
    ],
    "globals": {
        "Atomics": "readonly",
        "SharedArrayBuffer": "readonly"
    },
    "parserOptions": {
        "ecmaVersion": 2020,
        "sourceType": "module"
    },
    "plugins": [
        "vue"
    ],
    "rules": {
        "quotes": ["warn", "double"],
        "indent": ["error", 4],
        // "semi": ["warn", "always"],
        "comma-dangle": ["warn", "never"],
        "eol-last": ["warn", "always"],
    },
    "ignorePatterns": ["node_modules/**/*", "src/**/*", "mona/**/*", "loaders/**/*"],
};