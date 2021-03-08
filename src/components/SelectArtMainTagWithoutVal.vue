<template>
    <el-select
        :value="value"
        @input="$emit('input', $event)"
    >
        <el-option
            v-for="tagData in validTags"
            :key="tagData.name"
            :label="tagData.chs"
            :value="tagData.name"
        >
        </el-option>
        <el-option
            label="任意"
            value="any"
        >
        </el-option>
    </el-select>
</template>

<script>
import { artifactsTagMap } from "@asset/artifacts";
import { secondaryTags } from "@asset/tags";

const tagData = {
    flower: artifactsTagMap["flower"].map(item => secondaryTags[item]),
    feather: artifactsTagMap["feather"].map(item => secondaryTags[item]),
    sand: artifactsTagMap["sand"].map(item => secondaryTags[item]),
    cup: artifactsTagMap["cup"].map(item => secondaryTags[item]),
    head: artifactsTagMap["head"].map(item => secondaryTags[item]),
}

export default {
    name: "SelectArtMainTagWithoutValue",
    props: {
        value: {
            type: String,
            required: true,
        },
        position: {
            type: String,
        }
    },
    methods: {
    },
    computed: {
        // valid tags corespond to position
        validTags() {
            if (this.position === "") {
                return [];
            }

            return tagData[this.position];
        },
    },
    watch: {
        position() {
            if (artifactsTagMap[this.position].indexOf(this.value.name) === -1) {
                // the last selected tag does not exist on the new position
                let autoSelectedTagName = artifactsTagMap[this.position][0];
                this.$emit("input", autoSelectedTagName);
            }
        },
    }
}
</script>

<style scoped>
</style>