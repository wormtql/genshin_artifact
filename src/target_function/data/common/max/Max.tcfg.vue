<template>
    <div>
        <div class="config-item">
            <h3 class="config-title">伤害类型</h3>
            <select-element-type
                v-model="element"
            ></select-element-type>
        </div>

        <div class="config-item">
            <h3 class="config-title">技能类型</h3>
            <select-skill-type
                v-model="skill"
            ></select-skill-type>
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
    data() {
        return {
            element: "fire",
            skill: "a",
        }
    },
    methods: {
        setData(d) {
            this.element = d.element;
            this.skill = d.skill;
        }
    }
}
</script>