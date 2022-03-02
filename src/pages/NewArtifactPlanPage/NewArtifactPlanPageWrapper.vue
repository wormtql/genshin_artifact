<template>
    <new-artifact-plan-page></new-artifact-plan-page>
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
    }
}
</script>

<style scoped>

</style>