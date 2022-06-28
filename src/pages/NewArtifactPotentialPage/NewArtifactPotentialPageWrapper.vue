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
</script>

<style scoped>

</style>