<template>
    <mona-playground-page></mona-playground-page>
</template>

<script setup lang="ts">
import SimpleLoading from "@/components/loading/SimpleLoading.vue"
import SimpleError from "@/components/loading/SimpleError.vue"

import { loadScript } from "@/utils/common"

async function loadMonaco() {
    const monacoBase = "https://s1.pstatp.com/cdn/expire-1-y/monaco-editor/0.31.1/min/vs"
    await loadScript(monacoBase + "/loader.min.js")
    // @ts-ignore
    window.require.config({ paths: { vs: monacoBase } })
    // @ts-ignore
    window.MonacoEnvironment = {
        getWorkerUrl(workerId: any, label: any) {
            return `data:text/javascript;charset=utf-8,${encodeURIComponent(`
                self.MonacoEnvironment = {
                    baseUrl: "${monacoBase}"
                }
                importScripts("${monacoBase + '/base/worker/workerMain.js'}")
            `)}`
        }
    }
    return await new Promise((resolve, reject) => {
        window.require(["vs/editor/editor.main"], function () {
            // console.log(monaco)
            // @ts-ignore
            resolve(window.monaco)
        })
    })
}


const MonaPlaygroundPage = defineAsyncComponent({
    loader: () => loadMonaco().then(() => import(
        /* webpackChunkName: "playground-page" */
        /* webpackPrefetch: true */
        "./MonaPlaygroundPage.vue"
    )),
    loadingComponent: SimpleLoading,
    errorComponent: SimpleError,
    timeout: 60000,
})
</script>

<style scoped>

</style>