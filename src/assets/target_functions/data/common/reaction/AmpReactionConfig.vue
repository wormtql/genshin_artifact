<template>
    <div>
        <div class="config-item">
            <h3 class="config-title">伤害类型（指谁触发了元素反应）</h3>
            <select-element-type v-model="element"></select-element-type>
        </div>
        <div class="config-item">
            <h3 class="config-title">技能类型</h3>
            <select-skill-type v-model="skill"></select-skill-type>
        </div>
        <!-- <div class="config-item">
            <h3 class="config-title">反应类型</h3>
            <el-radio-group v-model="reactionType">
                <el-radio-button label="melt">融化</el-radio-button>
                <el-radio-button label="vaporize">蒸发</el-radio-button>
            </el-radio-group>

            <el-radio-group v-model="reactionType">
                <el-radio-button label="melt">融化</el-radio-button>
                <el-radio-button label="vaporize">蒸发</el-radio-button>
            </el-radio-group>
        </div> -->
    </div>
</template>

<script>
import SelectElementType from "@c/select/SelectElementType";
import SelectSkillType from "@c/select/SelectSkillType";

export default {
    name: "AmpReactionConfig",
    components: {
        SelectElementType,
        SelectSkillType,
    },
    data: function () {
        return {
            skill: "a",
            element: "fire",

            // reactionType: "melt",
        }
    }
}
</script>