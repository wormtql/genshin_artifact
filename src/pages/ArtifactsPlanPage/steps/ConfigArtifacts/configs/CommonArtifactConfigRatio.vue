<template>
    <wrapper :artifact-name="artifactName">
        <div class="buff-config-item">
            <h4 class="buff-config-title">{{ title }}</h4>
            <el-slider
                :value="value"
                @input="$emit('input', $event)"
                :min="0"
                :max="max"
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
        title: { default: "4件套效果比例" },
        max: { default: 1 }
    },
}
</script>