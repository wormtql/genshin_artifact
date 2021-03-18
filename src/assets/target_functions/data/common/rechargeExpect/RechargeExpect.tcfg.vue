<template>
    <div>
        <div class="config-item">
            <h3 class="config-title">元素充能效率</h3>
            <input-with-append v-model="value.recharge"></input-with-append>
        </div>
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
import InputWithAppend from "@c/InputWithAppend";

export default {
    name: "RechargeExpect.tcfg",
    components: {
        SelectElementType,
        SelectSkillType,
        InputWithAppend,
    },
    props: ["value"],
    first() {
        return {
            element: "fire",
            skill: "a",
            recharge: "120",
        }
    },
    compact(obj) {
        return {
            element: obj.element,
            skill: obj.skill,
            recharge: (parseFloat(obj.recharge) ?? 100) / 100,
        }
    }
}
</script>