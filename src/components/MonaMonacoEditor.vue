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

const languageName = "monaDSL"
monaco.languages.register({ id: languageName })
monaco.languages.setMonarchTokensProvider(languageName, {
    characters:[
        'Amber'

    ],
    tokenizer: {
        root: [
            [/prop|dmg/, "keyword"],
            [/\d+(\.\d+)?/, "number"],
            [/transformative|electro_charged|swirl_cryo|swirl_pyro|swirl_hydro|swirl_electro|overload|shatter|superconduct|super_conduct|expect|expectation|critical|crit|non_critical|non_crit|recharge|em|atk|def|hp|crit0/, "prop"],
            [/print|min|max|type/, "global"],
            [/[a-zA-Z]\w*/, {
				cases: {
                    '@characters':'characters',
					'@default': 'identifier'
				}
			}]
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
        { token: "global", foreground: "A5C261" },
        {token:"characters",foreground:"99ff00"}
    ],
    colors: {
        'editor.foreground': '#CCCCCC'
    }
})

function createDependencyProposals(range: { startLineNumber: number; endLineNumber: number; startColumn: number; endColumn: number }) {
	// returning a static list of proposals, not even looking at the prefix (filtering is done by the Monaco editor),
	// here you could do a server side lookup
	return [
		{
			label: 'spread',
			kind: monaco.languages.CompletionItemKind.Function,
			documentation: 'The Lodash library exported as Node.js modules.',
			insertText: 'spread',
			range: range
		},
		{
			label: '"express"',
			kind: monaco.languages.CompletionItemKind.Function,
			documentation: 'Fast, unopinionated, minimalist web framework',
			insertText: '"express": "*"',
			range: range
		},
		{
			label: '"mkdirp"',
			kind: monaco.languages.CompletionItemKind.Function,
			documentation: 'Recursively mkdir, like <code>mkdir -p</code>',
			insertText: '"mkdirp": "*"',
			range: range
		},
		{
			label: '"my-third-party-library"',
			kind: monaco.languages.CompletionItemKind.Function,
			documentation: 'Describe your library here',
			insertText: '"${1:my-third-party-library}": "${2:1.2.3}"',
			insertTextRules: monaco.languages.CompletionItemInsertTextRule.InsertAsSnippet,
			range: range
		}
	];
}

monaco.languages.registerCompletionItemProvider(languageName, {
	provideCompletionItems: function (model, position) {
		// find out if we are completing a property in the 'dependencies' object.
		var textUntilPosition = model.getValueInRange({
			startLineNumber: 1,
			startColumn: 1,
			endLineNumber: position.lineNumber,
			endColumn: position.column
		});
		var match = textUntilPosition.match(
			/Amber\..*/
		);
		if (!match) {	return { suggestions: [] };}
		var word = model.getWordUntilPosition(position);
        window.alert(textUntilPosition)
		var range = {
			startLineNumber: position.lineNumber,
			endLineNumber: position.lineNumber,
			startColumn: word.startColumn,
			endColumn: word.endColumn
		};
		return {
			suggestions: createDependencyProposals(range)
		};
	}
});
</script>

<style scoped lang="scss">
.root {
    height: 100%;
    //height: 300px;
    width: 100%;
}
</style>