<template>
    <div>
        <el-table
            :data="tableData"
            size="small"
        >
            <el-table-column
                prop="name"
                label="类型"
            ></el-table-column>
            <el-table-column
                prop="expectation"
                label="期望伤害"
            ></el-table-column>
            <el-table-column
                prop="critical"
                label="暴击伤害"
            ></el-table-column>
            <el-table-column
                prop="nonCritical"
                label="非暴击伤害"
            ></el-table-column>
        </el-table>
    </div>
</template>

<script>
export default {
    name: "DamageList",
    props: {
        analysisFromWasm: {}
    },
    data() {
        return {
            showDialog: false
        }
    },
    methods: {
        // handleClickItem(name) {
        //     this.showDialog = true
        //     const analysis = this.damageList[name]

        //     this.$nextTick(() => {
        //         const component = this.$refs["damageAnalysis"]
        //         component.setValue(analysis)
        //     })
        // }
    },
    computed: {
        element() {
            return this.analysisFromWasm.element
        },

        normalDamageTitle() {
            if (this.analysisFromWasm.is_heal) {
                return "治疗"
            } else {
                const map = {
                    "Pyro": "火元素伤害",
                    "Hydro": "水元素伤害",
                    "Electro": "雷元素伤害",
                    "Cryo": "冰元素伤害",
                    "Dendro": "草元素伤害",
                    "Geo": "岩元素伤害",
                    "Anemo": "风元素伤害",
                    "Physical": "物理伤害",
                }
                return map[this.element]
            }
        },

        tableData() {
            let temp = []
            const NO_DATA = "无数据"

            const r = (x) => Math.round(x)

            temp.push({
                expectation: r(this.analysisFromWasm.normal?.expectation) ?? NO_DATA,
                critical: r(this.analysisFromWasm.normal?.critical) ?? NO_DATA,
                nonCritical: r(this.analysisFromWasm.normal?.non_critical) ?? NO_DATA,
                name: this.normalDamageTitle
            })

            if (this.analysisFromWasm.melt) {
                temp.push({
                    expectation: r(this.analysisFromWasm.melt?.expectation) ?? NO_DATA,
                    critical: r(this.analysisFromWasm.melt?.critical) ?? NO_DATA,
                    nonCritical: r(this.analysisFromWasm.melt?.non_critical) ?? NO_DATA,
                    name: "融化"
                })
            }
            if (this.analysisFromWasm.vaporize) {
                temp.push({
                    expectation: r(this.analysisFromWasm.vaporize?.expectation) ?? NO_DATA,
                    critical: r(this.analysisFromWasm.vaporize?.critical) ?? NO_DATA,
                    nonCritical: r(this.analysisFromWasm.vaporize?.non_critical) ?? NO_DATA,
                    name: "蒸发"
                })
            }

            return temp
        }
    }
}
</script>

<style scoped lang="scss">
.item {
    height: 32px;
    display: flex;
    justify-content: space-between;
    align-items: center;
    font-size: 14px;

    &:hover {
        background-color: rgb(241, 241, 241);
    }

    .name {
        
    }

    .numbers {
        display: flex;
        gap: 4px;
    }

    .number {
        padding: 4px;
        border-radius: 3px;
    }

    .melt {
        color: rgb(63, 63, 63);
        // background-color: rgb(155, 218, 255);
        background-image: url("@image/misc/cryo");
        // background-size: 48px;
        background-position-x: -20px;
        background-position-y: -30px;
        background-repeat: no-repeat;
    }

    .pyro {
        color: rgb(255, 95, 95);
        background-color: rgb(255, 224, 224);
    }

    .physical {
        color: rgb(71, 71, 71);
        background-color: rgb(218, 218, 218);
    }
}
</style>