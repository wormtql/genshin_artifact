<template>
    <mona-playground-page></mona-playground-page>
</template>

<script>
import SimpleLoading from "@c/loading/SimpleLoading"
import SimpleError from "@c/loading/SimpleError"

import { requestMonaWasm } from "@/vendors/mona"
import { loadScript } from "@util/common"

const monacoBase = "https://s1.pstatp.com/cdn/expire-1-y/monaco-editor/0.31.1/min/vs"

function loadMonaco() {
    return loadScript(monacoBase + "/loader.min.js").then(() => {
        window.require.config({ paths: { vs: monacoBase } })

        window.MonacoEnvironment = {
            getWorkerUrl(workerId, label) {
                return `data:text/javascript;charset=utf-8,${encodeURIComponent(`
                    self.MonacoEnvironment = {
                        baseUrl: "${monacoBase}"
                    }
                    importScripts("${monacoBase + '/base/worker/workerMain.js'}")
                `)}`
            }
        }

        return new Promise((resolve, reject) => {
            window.require(["vs/editor/editor.main"], function () {
                // console.log(monaco)
                resolve(window.monaco)
            })
        })
    })
}

const MonaPlaygroundPage = () => {
    const component = Promise.all([loadMonaco(), requestMonaWasm()]).then(() => import(
        /* webpackChunkName: "playground-page" */
        /* webpackPrefetch: true */
        "./MonaPlaygroundPage"
        ))

    return {
        component: component,
        loading: SimpleLoading,
        error: SimpleError,
        timeout: 60000,
        delay: 0,
    }
}

export default {
    name: "MonaPlaygroundPageWrapper",
    components: {
        MonaPlaygroundPage
    }
}
</script>

<style scoped>

</style>