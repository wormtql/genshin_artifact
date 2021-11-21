<template>
    <div>
        <el-divider>梯度分析</el-divider>
        <el-alert
            type="warning"
            title="此处仅根据攻击力、暴击率、暴击伤害进行计算，仅供参考"
            :closable="false"
        ></el-alert>
        <div style="height: 300px">
            <v-chart :option="graphOption"></v-chart>
        </div>
        <p style="text-align: center">{{ analysisText }}</p>

        <div>
            <p
                class="item"
                v-show="howMuchBonusPerTag.bonus_S>0"
            >每个攻击力词条提升输出：{{ x100(howMuchBonusPerTag["bonus_S"]) }}%</p>
            <p
                class="item"
                v-show="howMuchBonusPerTag.bonus_p>0"
            >每个%攻击力词条提升输出：{{ x100(howMuchBonusPerTag["bonus_p"]) }}%</p>
            <p
                class="item"
                v-show="howMuchBonusPerTag.bonus_c>0"
            >每个暴击率词条提升输出：{{ x100(howMuchBonusPerTag["bonus_c"]) }}%</p>
            <p
                class="item"
                v-show="howMuchBonusPerTag.bonus_D>0"
            >每个暴击伤害词条提升输出：{{ x100(howMuchBonusPerTag["bonus_D"]) }}%</p>
             <p
                class="item"
                v-show="howMuchBonusPerTag.bonus_hpp>0"
            >每个%生命词条提升输出：{{ x100(howMuchBonusPerTag["bonus_hpp"]) }}%</p>
            <p
                class="item"
                v-show="howMuchBonusPerTag.bonus_hps>0"
            >每个生命词条提升输出：{{ x100(howMuchBonusPerTag["bonus_hps"]) }}%</p>
           
            <p
                class="item"
                v-show="howMuchBonusPerTag.bonus_em>0"
            >每个精通词条提升输出：{{ x100(howMuchBonusPerTag["bonus_em"]) }}%</p>
            <p
                class="item"
                v-show="howMuchBonusPerTag.bonus_dfs>0"
            >每个防御力词条提升输出：{{ x100(howMuchBonusPerTag["bonus_dfs"]) }}%</p>
            <p
                class="item"
                v-show="howMuchBonusPerTag.bonus_dfp>0"
            >每%防御力词条提升输出：{{ x100(howMuchBonusPerTag["bonus_dfp"]) }}%</p>
            <p
                class="item"
                v-show="howMuchBonusPerTag.bonus_recharge>0"
            >每个充能词条提升输出：{{ x100(howMuchBonusPerTag["bonus_recharge"]) }}%</p>
           </div>
    </div>
</template>

<script>
export default {
    name: "AttributeStatistics",
    props: ["attribute","config","arts",'howMuchBonusPerTag'],
    methods: {
        x100(value) {
            return (value * 100).toFixed(1);
        }
    },
    computed: {

        gradient() {
            let c = Math.min(this.attribute.critical, 1);
            let D = this.attribute.criticalDamage;
            let p = this.attribute.attackPercentage / this.attribute.attackBasic;
            let B = this.attribute.attackBasic;
            let S = this.attribute.attackStatic;
            // let gBaseAtk = (1 + c * D) * (1 + p);
            let gAtkPercentage = (1 + c * D) * B;
            let gAtkStatic = (1 + c * D);
            let gCritical = c < 1 ? (B + S + B * p) * D : 0;
            let gCriticalDamage = (B + S + B * p) * c;
            

            return {
                // "gBaseAtk": {
                //     chs: "基础攻击力（1）",
                //     value: gBaseAtk,
                // },
                "gAtkPercentage": {
                    chs: "攻击力%（1.5%）",
                    tagName: "attackPercentage",
                    value: gAtkPercentage / 66.67,
                },
                "gAtkStatic": {
                    chs: "攻击力（5）",
                    tagName: "attackStatic",
                    value: gAtkStatic * 5,
                },
                "gCritical": {
                    chs: "暴击率（1%）",
                    tagName: "critical",
                    value: gCritical / 100,
                },
                "gCriticalDamage": {
                    chs: "暴击伤害（2%）",
                    tagName: "criticalDamage",
                    value: gCriticalDamage / 50,
                }
            }
        },

        analysisText() {
            let max = Object.values(this.gradient).reduce((a, b) => {
                return a.value > b.value ? a : b;
            });
            return `接下来提升最大的词条：${max.chs}`;
        },

        graphOption() {
            let data = [];
            let g = this.gradient;
            // console.log(g);

            for (let item of Object.values(g)) {
                data.push({
                    name: item.chs,
                    value: item.value,
                })
            }

            return {
                tooltip: {
                    trigger: "item"
                },
                series: [
                    {
                        name: "属性梯度",
                        type: "pie",
                        // radius: ["40%", "70%"],
                        label: {
                            // show: false
                        },
                        itemStyle: {
                            borderRadius: 5,
                            borderColor: '#fff',
                            borderWidth: 2
                        },
                        data,
                    }
                ]
            }
        }
    }
}
</script>

<style scoped>
.item {
    text-align: center;
    margin: 0;
}
</style>