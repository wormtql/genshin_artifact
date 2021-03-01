<template>
    <div>
        <div class="config-item">
            <h3 class="config-title">伤害类型</h3>
            <select-element-type v-model="element"></select-element-type>
        </div>
        <div class="config-item">
            <h3 class="config-title">技能类型</h3>
            <select-skill-type v-model="skill"></select-skill-type>
        </div>
    </div>
</template>

<script>
import SelectElementType from "@c/SelectElementType";
import SelectSkillType from "@c/SelectSkillType";

export default {
    name: "ExpectConfig",
    components: {
        SelectElementType,
        SelectSkillType,
    },
    data: function () {
        return {
            element: "fire",
            skill: "a",
        }
    },
    methods: {
        compact() {
            return {
                element: this.element,
                skill: this.skill,
            }
        }
    }
}
</script>