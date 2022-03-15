<template>
    <el-input
        size="small"
        :value="value.value"
        @input="handleChangeValue"
    >
        <template slot="prepend">
            <select-artifact-main-stat
                :include-any="false"
                :position="position"
                :value="name"
                @input="handleChangeName"
                style="width: 150px"
            ></select-artifact-main-stat>
        </template>

        <template slot="append">
            <span v-if="isPercent">%</span>
        </template>
    </el-input>
</template>

<script>
import { artifactTags } from "@const/artifact"

import SelectArtifactMainStat from "@c/select/SelectArtifactMainStat"

export default {
    name: "InputArtifactMainStat",
    props: ["value", "position"],
    components: {
        SelectArtifactMainStat
    },
    methods: {
        handleChangeName(name) {
            console.log(name)
            if (name !== this.value.name) {
                let value = artifactTags[name].max[5]
                const isPercent = artifactTags[name].percentage
                if (isPercent) {
                    value *= 100
                }

                this.$emit("input", { name, value })
            }
        },

        handleChangeValue(value) {
            // value = parseFloat(value)
            // if (isNaN(value)) {
            //     value = 0
            // }
            // if (this.isPercent) {
            //     value = value / 100
            // }
            this.$emit("input", { name: this.value.name, value })
        }
    },
    computed: {
        isPercent() {
            if (!this.value.name) {
                return false
            }
            return artifactTags[this.value.name].percentage
        },

        name() {
            return this.value.name
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