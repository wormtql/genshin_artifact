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

// function f() {
//     let resolve1 = null
//     let reject1 = null
//
//     const promise = new Promise((resolve, reject) => {
//         resolve1 = resolve
//         reject1 = reject
//     })
//
//     return [resolve1, reject1, promise]
// }
//
// let [resolve, reject, promise] = f()

// export default {
//     // methods: {
//     //     handleCreated() {
//     //         resolve()
//     //     }
//     // },
//     // beforeRouteUpdate() {
//     //     console.log("update", this.$route.params)
//     // },
//     beforeRouteEnter(to, from, next) {
//         next(vm => {
//             // console.log("enter", vm.$route.params)
//
//             promise.then(() => {
//                 const component = vm.$refs["page"]
//                 // console.log(component)
//                 const presetName = vm.$route.params["presetName"]
//
//                 if (component && presetName) {
//                     component.usePreset(presetName)
//                 }
//             })
//
//         })
//     }
// }
</script>

<style scoped>

</style>