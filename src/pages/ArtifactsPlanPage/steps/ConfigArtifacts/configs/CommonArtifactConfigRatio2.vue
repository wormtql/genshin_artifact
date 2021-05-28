<template>
    <wrapper :artifact-name="artifactName">
        <div class="buff-config-item">
            <h4 class="buff-config-title">{{ title1 }}</h4>
            <el-slider
                :value="value[attrName1]"
                @input="handleUpdate1"
                :min="0"
                :max="max1"
                :step="0.01"
                show-input
                :show-input-controls="false"
                input-size="small"
            ></el-slider>
        </div>
        <div class="buff-config-item">
            <h4 class="buff-config-title">{{ title2 }}</h4>
            <el-slider
                :value="value[attrName2]"
                @input="handleUpdate2"
                :min="0"
                :max="max2"
                :step="0.01"
                show-input
                :show-input-controls="false"
                input-size="small"
            ></el-slider>
        </div>
    </wrapper>
</template>

<script>
import Wrapper from "./Wrapper";

export default {
    name: "CommonArtifactConfigRatio",
    components: {
        Wrapper,
    },
    props: {
        artifactName: {},
        value: {},
        title1: { default: "比例1" },
        title2: { default: "比例2" },
        max1: { default: 1 },
        max2: { default: 1 },
        attrName1: { default: "level" },
        attrName2: { default: "rate" }
    },
    methods: {
        handleUpdate1(value) {
            let v = {
                [this.attrName2]: this.value[this.attrName2],
                [this.attrName1]: value,
            };
            this.$emit("input", v);
        },

        handleUpdate2(value) {
            let v = {
                [this.attrName1]: this.value[this.attrName1],
                [this.attrName2]: value,
            };
            this.$emit("input", v);
        },
    }
}
</script>