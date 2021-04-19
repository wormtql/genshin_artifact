<template>
    <div>
        <div class="config-item">
            <h3 class="config-title">元素充能效率</h3>
            <input-with-append
                v-model="recharge"
            ></input-with-append>
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
import InputWithAppend from "@c/InputWithAppend";

export default {
    name: "RechargeExpect.tcfg",
    components: {
        SelectElementType,
        SelectSkillType,
        InputWithAppend,
    },
    data() {
        return {
            element: "fire",
            skill: "a",
            recharge: "120",
        }
    },
    methods: {
        compact() {
            return {
                element: this.element,
                skill: this.skill,
                recharge: (parseFloat(this.recharge) ?? 120) / 100,
            }
        },
        setData(d) {
            this.element = d.element;
            this.skill = d.skill;
            this.recharge = (d.recharge * 100).toString();
        }
    }
    
}
</script>