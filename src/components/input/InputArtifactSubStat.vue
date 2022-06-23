<template>
    <el-input
        :model-value="modelValue.value"
        @update:modelValue="handleChangeValue"
    >
        <template #prepend>
            <select-artifact-sub-stat
                :model-value="modelValue.name"
                @update:modelValue="handleChangeName"
                style="width: 100px"
            ></select-artifact-sub-stat>
        </template>

        <template v-if="isPercent" #append>
            <span>%</span>
        </template>
    </el-input>
</template>

<script>
import { artifactTags } from "@const/artifact"

import SelectArtifactSubStat from "@c/select/SelectArtifactSubStat"

export default {
    name: "InputArtifactSubStat",
    props: ["modelValue"],
    emits: ["update:modelValue"],
    components: {
        SelectArtifactSubStat
    },
    methods: {
        handleChangeName(name) {
            let temp = {
                name,
                value: this.modelValue.value
            }

            this.$emit("update:modelValue", temp)
        },

        handleChangeValue(value) {
            this.$emit("update:modelValue", { name: this.modelValue.name, value })
        }
    },
    computed: {
        isPercent() {
            if (!this.modelValue.name) {
                return false
            }
            return artifactTags[this.modelValue.name].percentage
        }
    }
}
</script>

<style scoped>

</style>