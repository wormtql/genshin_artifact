<template>
    <artifact-display
        :item="$store.getters['artifacts/artifactsById'][artifactId]"
        v-bind="$attrs"
    ></artifact-display>
</template>

<script>
import ArtifactDisplay from "@c/display/ArtifactDisplay"

export default {
    name: "ArtifactDisplayById",
    props: ["artifactId"],
    components: {
        ArtifactDisplay
    }
}
</script>

<style scoped>

</style>