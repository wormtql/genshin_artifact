<template>
    <wrapper :artifact-name="artifactName">
        <div class="buff-config-item">
            <h3 class="buff-config-title">{{ title1 }}</h3>
            <select-element-type
                :elements="elements"
                :value="value.element"
                @input="updateElement"
            ></select-element-type>
        </div>

        <div class="buff-config-item">
            <h3 class="buff-config-title">{{ title2 }}</h3>
            <el-slider
                :value="value.rate"
                @input="updateRate"
                :min="0"
                :max="max"
                :step="0.01"
                show-input
                :show-input-controls="false"
            ></el-slider>
        </div>
    </wrapper>
</template>

<script>
import Wrapper from "./Wrapper";
import SelectElementType from "@c/select/SelectElementType";

export default {
    name: "CommonArtifactConfigRatio",
    components: {
        Wrapper,
        SelectElementType,
    },
    props: {
        artifactName: {},
        title1: { default: "触发元素" },
        title2: { default: "4件套效果触发比例" },
        value: {},
        max: { default: 1 },
        elements: { default: () => ["fire", "grass", "ice", "thunder", "water", "wind", "rock"] },
    },
    methods: {
        updateElement(element) {
            let value = {
                element,
                rate: this.value.rate,
            };
            this.$emit("input", value);
        },

        updateRate(rate) {
            let value = {
                element: this.value.element,
                rate,
            }
            this.$emit("input", value);
        }
    }
}
</script>