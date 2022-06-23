<template>
    <el-input
        :model-value="modelValue.value"
        @update:modelValue="handleChangeValue"
    >
        <template #prepend>
            <select-artifact-main-stat
                :include-any="false"
                :position="position"
                :model-value="name"
                @update:modelValue="handleChangeName"
                style="width: 100px"
            ></select-artifact-main-stat>
        </template>

        <template v-if="isPercent" #append>
            <span>%</span>
        </template>
    </el-input>
</template>

<script>
import { artifactTags } from "@const/artifact"

import SelectArtifactMainStat from "@c/select/SelectArtifactMainStat"

export default {
    name: "InputArtifactMainStat",
    props: ["modelValue", "position"],
    emits: ["update:modelValue"],
    components: {
        SelectArtifactMainStat
    },
    methods: {
        handleChangeName(name) {
            if (name !== this.modelValue.name) {
                let value = artifactTags[name].max[5]
                const isPercent = artifactTags[name].percentage
                if (isPercent) {
                    value *= 100
                }

                this.$emit("update:modelValue", { name, value })
            }
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
        },

        name() {
            return this.modelValue.name
        },

        // displayedValue() {
        //     if (this.isPercent) {
        //         return (this.value.value * 100)
        //     } else {
        //         return this.value.value
        //     }
        // }
    }
}
</script>

<style scoped>

</style>