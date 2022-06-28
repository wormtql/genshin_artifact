<template>
    <div>
        <suspense>
            <team-optimization-page></team-optimization-page>
            <template #fallback>
                <simple-loading></simple-loading>
            </template>
        </suspense>
    </div>

</template>

<script setup lang="ts">
import SimpleLoading from "@/components/loading/SimpleLoading.vue"
import SimpleError from "@/components/loading/SimpleError.vue"

const TeamOptimizationPage = defineAsyncComponent({
    loader: () => import(
        /* webpackChunkName: "team-optimization-page" */
        /* webpackPrefetch: true */
        "./TeamOptimizationPage.vue"
    ),
    loadingComponent: SimpleLoading,
    errorComponent: SimpleError,
    timeout: 30000,
})
</script>

<style scoped>

</style>