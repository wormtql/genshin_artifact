<template>
    <new-artifact-plan-page></new-artifact-plan-page>
</template>

<script>
import SimpleLoading from "@c/loading/SimpleLoading"
import SimpleError from "@c/loading/SimpleError"

import { requestMonaWasm } from "@/vendors/mona"

const NewArtifactPlanPage = () => {
    const component = requestMonaWasm().then(() => import(
        /* webpackChunkName: "artifact-plan-page" */
        /* webpackPrefetch: true */
        "./NewArtifactPlanPage"
        )
    )

    return {
        component,
        loading: SimpleLoading,
        error: SimpleError,
        timeout: 3000,
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