<template>
    <div>
        <div class="config-item">
            <h3 class="config-title">阈值</h3>
            <el-input
                v-model="threshold"
                size="small"
            ></el-input>
        </div>

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
    name: "CritExpect.tcfg",
    components: {
        SelectElementType,
        SelectSkillType,
    },
    data() {
        return {
            threshold: "0.5",
            element: "fire",
            skill: "a",
        }
    },
    methods: {
        compact() {
            return {
                threshold: parseFloat(this.threshold) ?? 0.5,
                element: this.element,
                skill: this.skill,
            }
        },
        setData(d) {
            this.threshold = d.threshold.toString();
            this.element = d.element;
            this.skill = d.skill;
        }
    }
    
}
</script>