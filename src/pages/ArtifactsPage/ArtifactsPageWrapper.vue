<template>
    <artifacts-page></artifacts-page>
</template>

<script setup lang="ts">
import SimpleLoading from "@/components/loading/SimpleLoading.vue"
import SimpleError from "@/components/loading/SimpleError.vue"

const ArtifactsPage = defineAsyncComponent({
    loader: () => import(/* webpackChunkName: "artifacts-page" */ "./ArtifactsPage.vue"),
    errorComponent: SimpleError,
    loadingComponent: SimpleLoading,
    delay: 0
})
</script>

<style scoped>

</style>