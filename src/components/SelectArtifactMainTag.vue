<template>
    <el-input
        :value="value.value"
        @input="handleValueChange"
        size="small"
    >
        <template slot="prepend">
            <el-select
                :value="value.name"
                @input="handleTagChange"
                class="prepend"
            >
                <el-option
                    v-for="tagData in validTags"
                    :key="tagData.name"
                    :label="tagData.chs"
                    :value="tagData.name"
                >
                </el-option>
            </el-select>
        </template>

        <template slot="append">
            <span v-if="isPercentage">%</span>
        </template>
    </el-input>
</template>

<script>
import { artifactsTagMap } from "@asset/artifacts";
import { artifactTags } from "@const/artifact";
// import { convertDisplayTagValue } from "@util/utils";

const tagData = {
    flower: artifactsTagMap["flower"].map(item => artifactTags[item]),
    feather: artifactsTagMap["feather"].map(item => artifactTags[item]),
    sand: artifactsTagMap["sand"].map(item => artifactTags[item]),
    cup: artifactsTagMap["cup"].map(item => artifactTags[item]),
    head: artifactsTagMap["head"].map(item => artifactTags[item]),
}

export default {
    name: "SelectArtifactMainTag",
    props: {
        value: {
            type: Object,
            required: true,
        },
        position: {
            type: String,
        },
    },
    methods: {
        handleTagChange(event) {
            let temp = Object.assign({}, this.value);
            temp.name = event;

            this.$emit("input", temp);
        },

        handleValueChange(event) {
            let temp = Object.assign({}, this.value);
            temp.value = event;

            this.$emit("input", temp);
        }
    },
    computed: {
        // valid tags corespond to position
        validTags() {
            if (this.position === "") {
                return [];
            }

            return tagData[this.position];
        },

        isPercentage() {
            if (!this.value.name) {
                return false;
            }
            return artifactTags[this.value.name].percentage;
        },
    },
    watch: {
        position() {
            if (artifactsTagMap[this.position].indexOf(this.value.name) === -1) {
                // the last selected tag does not exist on the new position
                let autoSelectedTagName = artifactsTagMap[this.position][0];
                let temp = Object.assign({}, this.value);
                temp.name = autoSelectedTagName;
                this.$emit("input", temp);
            }
        },

        // "value.name"() {
        //     let maxValue = artifactTags[this.value.name].max[5];
        //     let temp = Object.assign({}, this.value);

        //     temp.value = convertDisplayTagValue(temp.name, maxValue);
        //     this.$emit("input", temp);
        // }
    }
}
</script>

<style scoped>
.prepend {
    width: 150px;
}
</style>