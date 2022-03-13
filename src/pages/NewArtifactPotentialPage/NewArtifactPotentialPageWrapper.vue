<template>
    <new-artifact-potential-page></new-artifact-potential-page>
</template>

<script>
import SimpleLoading from "@c/loading/SimpleLoading"
import SimpleError from "@c/loading/SimpleError"

import { requestMonaWasm } from "@/vendors/mona"

const NewArtifactPotentialPage = () => {
    const component = requestMonaWasm().then(() => import(
        /* webpackChunkName: "artifact-potential-page" */
        /* webpackPrefetch: true */
        "./NewArtifactPotentialPage"
        )
    )

    return {
        component,
        loading: SimpleLoading,
        error: SimpleError,
        timeout: 30000,
        delay: 0,
    }
}

export default {
    name: "NewArtifactPotentialPageWrapper",
    components: {
        NewArtifactPotentialPage
    }
}
</script>

<style scoped>

</style>