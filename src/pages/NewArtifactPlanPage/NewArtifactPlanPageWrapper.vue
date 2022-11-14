<template>
    <div>
        <suspense>
            <new-artifact-plan-page></new-artifact-plan-page>

            <template #fallback>
                <simple-loading></simple-loading>
            </template>
        </suspense>
    </div>
</template>

<script setup lang="ts">
import SimpleLoading from "@/components/loading/SimpleLoading.vue"
import SimpleError from "@/components/loading/SimpleError.vue"


const NewArtifactPlanPage = defineAsyncComponent({
    loader: () => import(
        /* webpackChunkName: "artifact-plan-page" */
        /* webpackPrefetch: true */
        "./NewArtifactPlanPage.vue"
    ),
    loadingComponent: SimpleLoading,
    errorComponent: SimpleError,
    delay: 0,
    timeout: 60000
})
</script>

<style scoped>

</style>