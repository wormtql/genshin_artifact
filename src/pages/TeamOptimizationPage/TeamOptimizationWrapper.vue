<template>
    <team-optimization-page></team-optimization-page>
</template>

<script>
import { requestMonaWasm } from "@/vendors/mona"

import SimpleLoading from "@c/loading/SimpleLoading"
import SimpleError from "@c/loading/SimpleError"

const TeamOptimizationPage = () => {
    const component = requestMonaWasm().then(() => import(
        /* webpackChunkName: "team-optimization-page" */
        /* webpackPrefetch: true */
        "./TeamOptimizationPage"
        )
    )

    return {
        component,
        loading: SimpleLoading,
        error: SimpleError,
        timeout: 30000,
    }
}

export default {
    name: "TeamOptimizationWrapper",
    components: { TeamOptimizationPage }
}
</script>

<style scoped>

</style>