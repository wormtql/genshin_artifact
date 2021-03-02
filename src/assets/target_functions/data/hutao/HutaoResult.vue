<template>
    <div>
        <el-row>
            <el-col :span="8">
                <p>开E后攻击力</p>
            </el-col>
            <el-col :span="16">
                <p>{{ atkE }}</p>
            </el-col>
        </el-row>
    </div>
</template>

<script>
import skill from "./skill";

export default {
    name: "HutaoResult",
    props: {
        attribute: {
            type: Object,
        },
        config: {
            type: Object,
        }
    },
    computed: {
        atkE() {
            let hp = this.attribute.lifeBasic + this.attribute.lifePercentage + this.attribute.lifeStatic;
            let atk = this.attribute.attackBasic + this.attribute.attackPercentage + this.attribute.attackStatic;
            let eLevel = this.config.cArgs.skill2;
            let atkInc = skill.e.hp[eLevel - 1];

            return hp * atkInc + atk;
        }
    }
}
</script>