<template>
    <div>
        <div class="config-item">
            <h3 class="config-title">属性名</h3>
            <el-radio-group
                v-model="fieldName"
                size="small"
            >
                <el-radio-button label="attack">攻击力</el-radio-button>
                <el-radio-button label="life">生命值</el-radio-button>
                <el-radio-button label="defend">防御力</el-radio-button>
                <el-radio-button label="criticalDamage">暴击伤害</el-radio-button>
                <el-radio-button label="critical">暴击率</el-radio-button>
                <el-radio-button label="elementalMastery">元素精通</el-radio-button>
                <el-radio-button label="recharge">元素充能效率</el-radio-button>
            </el-radio-group>
        </div>
    </div>
</template>

<script>
export default {
    name: "Single.tcfg",
    data() {
        return {
            fieldName: "attack",
        }
    },
    methods: {
        setData(d) {
            this.fieldName = d.fieldName;
        }
    }
}

</script>