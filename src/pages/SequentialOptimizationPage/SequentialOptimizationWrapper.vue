<template>
    <sequential-optimization-page></sequential-optimization-page>
</template>

<script setup lang="ts">
import SimpleLoading from "@/components/loading/SimpleLoading.vue"
import SimpleError from "@/components/loading/SimpleError.vue"

const SequentialOptimizationPage = defineAsyncComponent({
    loader: () => import(
        /* webpackChunkName: "sequential-optimization-page" */
        /* webpackPrefetch: true */
        "./SequentialOptimizationPage.vue"
    ),
    errorComponent: SimpleError,
    loadingComponent: SimpleLoading,
    delay: 0,
    timeout: 60000
})
</script>
