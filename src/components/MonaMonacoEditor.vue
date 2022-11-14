<template>
    <div ref="root" class="root"></div>
</template>

<script setup lang="ts">
let editor: any = null

function getValue() {
    const value = editor.getValue()
    return value
}

defineExpose({
    getValue
})

const root = ref<null | HTMLElement>(null)
onMounted(() => {
    // console.log(el)

    if (root.value) {
        editor = monaco.editor.create(root.value, {
            value: "dmg a = Amber.transformative\n" +
                "overload = a.overload\n" +
                "e = a.electro_charged\n" +
                "result = overload * 10 + e * 8\n" +
                "print(result)",
            // language: "javascript",
            language: languageName,
            theme: "myTheme"
        })
    }

})
</script>

<script lang="ts">
import * as monaco from "monaco-editor"
import * as charMeta from "@/assets/_gen_character"

let char_list: Array<string> = [];
for (const [key, val] of Object.entries(charMeta.default)) {
    char_list.push(val.name);
}

const languageName = "monaDSL"
monaco.languages.register({id: languageName})
monaco.languages.setMonarchTokensProvider(languageName, {
    characters: char_list,
    tokenizer: {
        root: [
            [/prop|dmg/, "keyword"],
            [/\d+(\.\d+)?/, "number"],
            [/normal|transformative|electro_charged|swirl_cryo|swirl_pyro|swirl_hydro|swirl_electro|overload|shatter|superconduct|super_conduct|spread|aggravate|bloom|hyperbloom|burgeon|burning|expect|expectation|critical|crit|non_critical|non_crit|recharge|em|atk|def|hp|crit0/, "prop"],
            [/print|min|max|type/, "global"],
            [/[a-zA-Z]\w*/, {
                cases: {
                    '@characters': 'characters',
                    '@default': 'identifier'
                }
            }],
            [/\/\/.*$/, "comment"]
        ]
    }
})

monaco.editor.defineTheme("myTheme", {
    base: "vs-dark",
    inherit: true,
    rules: [
        {token: "keyword", foreground: "FFA500"},
        {token: "number", foreground: "7A9EC2"},
        {token: "prop", foreground: "9E7BB0"},
        {token: "global", foreground: "A5C261"},
        {token: "characters", foreground: "99ff00"},
        {token: "identifier", foreground: "D3D3D3"}
    ],
    colors: {
        'editor.foreground': '#CCCCCC'
    }
})
</script>

<style scoped lang="scss">
.root {
    height: 100%;
    //height: 300px;
    width: 100%;
}
</style>