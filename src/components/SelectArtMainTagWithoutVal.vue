<template>
    <el-select
        :value="value"
        @input="$emit('input', $event)"
        size="small"
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
import { artifactTags } from "@const/artifact";

const tagData = {
    flower: artifactsTagMap["flower"].map(item => artifactTags[item]),
    feather: artifactsTagMap["feather"].map(item => artifactTags[item]),
    sand: artifactsTagMap["sand"].map(item => artifactTags[item]),
    cup: artifactsTagMap["cup"].map(item => artifactTags[item]),
    head: artifactsTagMap["head"].map(item => artifactTags[item]),
    "any": Object.values(artifactTags),
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