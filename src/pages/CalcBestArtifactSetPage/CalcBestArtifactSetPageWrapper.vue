<template>
    <div>
        <suspense>
            <page></page>

            <template #fallback>
                <simple-loading></simple-loading>
            </template>
        </suspense>
    </div>
</template>

<script setup lang="ts">
import SimpleLoading from "@/components/loading/SimpleLoading.vue"
import SimpleError from "@/components/loading/SimpleError.vue"

const Page = defineAsyncComponent({
    loader: () => import(
        /* webpackChunkName: "artifact-best-set-page" */
        /* webpackPrefetch: true */
        "./CalcBestArtifactSetPage.vue"
        ),
    loadingComponent: SimpleLoading,
    errorComponent: SimpleError,
    timeout: 60000,
})
</script>

<style scoped>

</style>