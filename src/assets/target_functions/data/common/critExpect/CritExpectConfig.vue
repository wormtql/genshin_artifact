<template>
    <div>
        <div class="config-item">
            <h3 class="config-title">阈值</h3>
            <el-input v-model="threshold"></el-input>
        </div>

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
import SelectElementType from "@c/select/SelectElementType";
import SelectSkillType from "@c/select/SelectSkillType";

export default {
    name: "CritExpectConfig",
    components: {
        SelectElementType,
        SelectSkillType,
    },
    data: function () {
        return {
            threshold: "0.5",
            element: "fire",
            skill: "a",
        }
    },
    methods: {
        compact() {
            return {
                threshold: parseFloat(this.threshold),
                element: this.element,
                skill: this.skill,
            }
        }
    }
}
</script>