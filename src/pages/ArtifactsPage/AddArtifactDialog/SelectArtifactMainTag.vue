<template>
    <el-input
        :model-value="modelValue.value"
        @update:modelValue="handleValueChange"
    >
        <template #prepend>
            <el-select
                :model-value="modelValue.name"
                @update:modelValue="handleTagChange"
                class="prepend"
            >
                <el-option
                    v-for="tagData in validTags"
                    :key="tagData.name"
                    :label="t('stat', tagData.name)"
                    :value="tagData.name"
                >
                </el-option>
            </el-select>
        </template>

        <template v-if="isPercentage" #append>
            <span>%</span>
        </template>
    </el-input>
</template>

<script>
import { artifactsTagMap } from "@asset/artifacts";
import { artifactTags } from "@const/artifact";
import {useI18n} from "@/i18n/i18n";
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
        modelValue: {
            type: Object,
            required: true,
        },
        position: {
            type: String,
        },
    },
    emits: ["update:modelValue"],
    methods: {
        handleTagChange(event) {
            let temp = Object.assign({}, this.modelValue);
            temp.name = event;

            this.$emit("update:modelValue", temp);
        },

        handleValueChange(event) {
            let temp = Object.assign({}, this.modelValue);
            temp.value = event;

            this.$emit("update:modelValue", temp);
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
            if (!this.modelValue.name) {
                return false;
            }
            return artifactTags[this.modelValue.name].percentage;
        },
    },
    watch: {
        position() {
            if (artifactsTagMap[this.position].indexOf(this.modelValue.name) === -1) {
                // the last selected tag does not exist on the new position
                let autoSelectedTagName = artifactsTagMap[this.position][0];
                let temp = Object.assign({}, this.modelValue);
                temp.name = autoSelectedTagName;
                this.$emit("update:modelValue", temp);
            }
        },
    },
    setup() {
        const { t } = useI18n()

        return {
            t
        }
    }
}
</script>

<style scoped>
.prepend {
    /*width: 100px;*/
    text-overflow: ellipsis;
}
</style>