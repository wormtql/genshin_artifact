<template>
    <new-artifact-plan-page ref="page"></new-artifact-plan-page>
</template>

<script>
import SimpleLoading from "@c/loading/SimpleLoading"
import SimpleError from "@c/loading/SimpleError"

import { requestMonaWasm } from "@/vendors/mona"

const NewArtifactPlanPage = () => {
    const component1 = requestMonaWasm().then(() => import(
        /* webpackChunkName: "artifact-plan-page" */
        /* webpackPrefetch: true */
        "./NewArtifactPlanPage"
        )
    )

    // const component = new Promise((resolve, reject) => {
    //     setTimeout(() => {
    //         resolve()
    //     }, 30000)
    // }).then(() => {
    //     return component1
    // })

    return {
        component: component1,
        loading: SimpleLoading,
        error: SimpleError,
        timeout: 30000,
        delay: 0,
    }
}

export default {
    name: "NewArtifactPlanPageWrapper",
    components: {
        NewArtifactPlanPage
    },
    // beforeRouteUpdate() {
    //     console.log("update", this.$route.params)
    // },
    beforeRouteEnter(to, from, next) {
        next(vm => {
            // console.log("enter", vm.$route.params)
            const component = vm.$refs["page"]
            const presetName = vm.$route.params["presetName"]

            if (component && presetName) {
                component.usePreset(presetName)
            }
        })
    }
}
</script>

<style scoped>

</style>