<template>
    <new-artifact-plan-page
        ref="page"
        @created="handleCreated"
    ></new-artifact-plan-page>
</template>

<script>
import SimpleLoading from "@c/loading/SimpleLoading"
import SimpleError from "@c/loading/SimpleError"

import { requestMonaWasm } from "@/vendors/mona"

const component = requestMonaWasm().then(() => import(
    /* webpackChunkName: "artifact-plan-page" */
    /* webpackPrefetch: true */
    "./NewArtifactPlanPage"
    )
)

const NewArtifactPlanPage = () => {
    // const component1 = requestMonaWasm().then(() => import(
    //     /* webpackChunkName: "artifact-plan-page" */
    //     /* webpackPrefetch: true */
    //     "./NewArtifactPlanPage"
    //     )
    // )

    // const component = new Promise((resolve, reject) => {
    //     setTimeout(() => {
    //         resolve()
    //     }, 30000)
    // }).then(() => {
    //     return component1
    // })

    return {
        // component: component1,
        component,
        loading: SimpleLoading,
        error: SimpleError,
        timeout: 60000,
        delay: 0,
    }
}

function f() {
    let resolve1 = null
    let reject1 = null

    const promise = new Promise((resolve, reject) => {
        resolve1 = resolve
        reject1 = reject
    })

    return [resolve1, reject1, promise]
}

let [resolve, reject, promise] = f()

export default {
    name: "NewArtifactPlanPageWrapper",
    components: {
        NewArtifactPlanPage
    },
    methods: {
        handleCreated() {
            // console.log("created")
            resolve()
        }
    },
    // beforeRouteUpdate() {
    //     console.log("update", this.$route.params)
    // },
    beforeRouteEnter(to, from, next) {
        next(vm => {
            // console.log("enter", vm.$route.params)

            promise.then(() => {
                const component = vm.$refs["page"]
                // console.log(component)
                const presetName = vm.$route.params["presetName"]

                if (component && presetName) {
                    component.usePreset(presetName)
                }
            })

        })
    }
}
</script>

<style scoped>

</style>