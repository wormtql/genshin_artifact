<template>
    <wrapper :artifact-name="artifactName">
        <div class="buff-config-item">
            <h3 class="buff-config-title">{{ title }}</h3>
            <select-element-multi
                :elements="elements"
                :value="value"
                @input="$emit('input', $event)"
            ></select-element-multi>
        </div>
    </wrapper>
</template>

<script>
import Wrapper from "./Wrapper";
import SelectElementMulti from "@c/select/SelectElementMulti";

export default {
    name: "CommonArtifactConfigEleMulti",
    components: {
        Wrapper,
        SelectElementMulti,
    },
    props: {
        artifactName: {},
        title: { default: "元素" },
        value: {},
        elements: { default: () => ["fire", "grass", "ice", "thunder", "water", "wind", "rock"] },
    },
}
</script>