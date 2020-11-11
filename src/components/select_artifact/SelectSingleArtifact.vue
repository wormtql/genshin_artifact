<template>
    <div class="root">
        <artifact-item
            v-for="(art, index) in data"
            :key="index"
            :item="art"
            :selected="index === value"
            @click="onArtClicked(index)"
            style="margin: 0 8px 8px 0"
        >
        </artifact-item>
    </div>
</template>

<script>
import ArtifactItem from "./ArtifactItem";

export default {
    name: "SelectSingleArtifact",
    components: {
        ArtifactItem
    },
    props: {
        data: {
            type: Array,
        },
        value: {
            type: Number
        }
    },
    methods: {
        onArtClicked(index) {
            // window.console.log(index);
            if (this.value === index) {
                this.$emit("input", -1);
            } else {
                this.$emit("input", index);
            }
        }
    }
}
</script>

<style scoped>
.root {
    display: flex;
    flex-wrap: wrap;
}
</style>