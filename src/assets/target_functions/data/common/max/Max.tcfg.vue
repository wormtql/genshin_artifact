<template>
    <div>
        <div class="config-item">
            <h3 class="config-title">伤害类型</h3>
            <select-element-type v-model="value.element"></select-element-type>
        </div>

        <div class="config-item">
            <h3 class="config-title">技能类型</h3>
            <select-skill-type v-model="value.skill"></select-skill-type>
        </div>
    </div>
</template>

<script>
import SelectElementType from "@c/select/SelectElementType";
import SelectSkillType from "@c/select/SelectSkillType";

export default {
    name: "Max.tcfg",
    components: {
        SelectElementType,
        SelectSkillType,
    },
    props: {
        value: {
            type: Object,
        }
    },
    first() {
        return {
            element: "fire",
            skill: "a",
        }
    },
}
</script>