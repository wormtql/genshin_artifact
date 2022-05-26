<template>
    <div ref="root" class="root"></div>
</template>

<script>
import * as monaco from "monaco-editor"
import {characterData} from "@character"

const languageName = "monaDSL"
monaco.languages.register({ id: languageName })
monaco.languages.setMonarchTokensProvider(languageName, {
    tokenizer: {
        root: [
            [/prop|dmg/, "keyword"],
            [/\d+(\.\d+)?/, "number"],
            [/transformative|electro_charged|swirl_cryo|swirl_pyro|swirl_hydro|swirl_electro|overload|shatter|superconduct|super_conduct|expect|expectation|critical|crit|non_critical|non_crit|recharge|em|atk|def|hp|crit0/, "prop"],
            [/print|min|max|type/, "global"]
        ]
    }
})
monaco.editor.defineTheme("myTheme", {
    base: "vs-dark",
    inherit: true,
    rules: [
        { token: "keyword", foreground: "FFA500" },
        { token: "number", foreground: "7A9EC2" },
        { token: "prop", foreground: "9E7BB0" },
        { token: "global", foreground: "A5C261" }
    ],
    colors: {
        'editor.foreground': '#CCCCCC'
    }
})

let editor = null

export default {
    name: "MonaMonacoEditor",
    methods: {
        getValue() {
            const value = editor.getValue()
            return value
        }
    },
    mounted() {
        const el = this.$refs["root"]

        editor = monaco.editor.create(el, {
            value: "dmg a = Amber.transformative\n" +
                "overload = a.overload\n" +
                "e = a.electro_charged\n" +
                "result = overload * 10 + e * 8\n" +
                "print(result)",
            language: languageName,
            theme: "myTheme"
        })
    },
}
</script>

<style scoped lang="scss">
.root {
    height: 100%;
    width: 100%;
}
</style>