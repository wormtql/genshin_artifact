<template>
    <el-input
        size="small"
        :value="value.value"
        @input="handleChangeValue"
    >
        <template slot="prepend">
            <select-artifact-sub-stat
                :value="value.name"
                @input="handleChangeName"
                style="width: 150px"
            ></select-artifact-sub-stat>
        </template>

        <template slot="append">
            <span v-if="isPercent">%</span>
        </template>
    </el-input>
</template>

<script>
import { artifactTags } from "@const/artifact"

import SelectArtifactSubStat from "@c/select/SelectArtifactSubStat"

export default {
    name: "InputArtifactSubStat",
    props: ["value"],
    components: {
        SelectArtifactSubStat
    },
    methods: {
        handleChangeName(name) {
            let temp = {
                name,
                value: this.value.value
            }

            this.$emit("input", temp)
        },

        handleChangeValue(value) {
            this.$emit("input", { name: this.value.name, value })
        }
    },
    computed: {
        isPercent() {
            if (!this.value.name) {
                return false
            }
            return artifactTags[this.value.name].percentage
        }
    }
}
</script>

<style scoped>

</style>