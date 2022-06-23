<template>
    <new-artifact-potential-page></new-artifact-potential-page>
</template>

<script setup lang="ts">
import SimpleLoading from "@/components/loading/SimpleLoading.vue"
import SimpleError from "@/components/loading/SimpleError.vue"

const NewArtifactPotentialPage = defineAsyncComponent({
    loader: () => import(
        /* webpackChunkName: "artifact-potential-page" */
        /* webpackPrefetch: true */
        "./NewArtifactPotentialPage.vue"
    ),
    errorComponent: SimpleError,
    loadingComponent: SimpleLoading,
    delay: 0,
    timeout: 60000
})

// const NewArtifactPotentialPage = () => {
//     const component = requestMonaWasm().then(() => import(
//         /* webpackChunkName: "artifact-potential-page" */
//         /* webpackPrefetch: true */
//         "./NewArtifactPotentialPage"
//         )
//     )
//
//     return {
//         component,
//         loading: SimpleLoading,
//         error: SimpleError,
//         timeout: 30000,
//         delay: 0,
//     }
// }

// export default {
//     name: "NewArtifactPotentialPageWrapper",
//     components: {
//         NewArtifactPotentialPage
//     }
// }
</script>

<style scoped>

</style>