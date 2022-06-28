<template>
    <mona-playground-page></mona-playground-page>
</template>

<script setup lang="ts">
import SimpleLoading from "@/components/loading/SimpleLoading.vue"
import SimpleError from "@/components/loading/SimpleError.vue"

import { loadScript } from "@/utils/common"
// import {useMona} from "@/wasm/mona"


import MonaPlaygroundPage from "./MonaPlaygroundPage.vue"


// const MonaPlaygroundPage = defineAsyncComponent({
//     loader: () => import(
//         /* webpackChunkName: "playground-page" */
//         /* webpackPrefetch: true */
//         "./MonaPlaygroundPage.vue"
//     ),
//     loadingComponent: SimpleLoading,
//     errorComponent: SimpleError,
//     timeout: 60000,
//     delay: 0
// })

// const MonaPlaygroundPage = () => {
//     const component = Promise.all([loadMonaco()]).then(() => import(
//         /* webpackChunkName: "playground-page" */
//         /* webpackPrefetch: true */
//         "./MonaPlaygroundPage"
//         ))
//
//     return {
//         component: component,
//         loading: SimpleLoading,
//         error: SimpleError,
//         timeout: 60000,
//         delay: 0,
//     }
// }

// export default {
//     name: "MonaPlaygroundPageWrapper",
//     components: {
//         MonaPlaygroundPage
//     }
// }
</script>

<style scoped>

</style>