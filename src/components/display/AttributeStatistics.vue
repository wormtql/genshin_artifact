<template>
    <div>
        <el-divider>梯度分析</el-divider>
        <div style="height: 300px">
            <v-chart :option="graphOption"></v-chart>
        </div>
        <p style="text-align: center">{{ analysisText }}</p>

        <div>
            <p
                class="item"
            >每个攻击力词条提升输出：{{ x100(howMuchBonusPerTag["bonus_S"].value) }}%</p>
            <p
                class="item"
            >每个%攻击力词条提升输出：{{ x100(howMuchBonusPerTag["bonus_p"].value) }}%</p>
            <p
                class="item"
            >每个暴击率词条提升输出：{{ x100(howMuchBonusPerTag["bonus_c"].value) }}%</p>
            <p
                class="item"
            >每个暴击伤害词条提升输出：{{ x100(howMuchBonusPerTag["bonus_D"].value) }}%</p>
        </div>
    </div>
</template>

<script>
import artifactEff from "@const/artifact_eff";

export default {
    name: "AttributeStatistics",
    props: ["attribute"],
    methods: {
        x100(value) {
            return (value * 100).toFixed(1);
        }
    },
    computed: {
        howMuchBonusPerTag() {
            let c = Math.min(this.attribute.critical, 1);
            let D = this.attribute.criticalDamage;
            let p = this.attribute.attackPercentage / this.attribute.attackBasic;
            let B = this.attribute.attackBasic;
            let S = this.attribute.attackStatic;

            let eff = artifactEff["5"];

            let bonus_S = (B + S + eff.attackStatic[3] + p * B) / (B + S + p * B);
            let bonus_p = (B + S + (p + eff.attackPercentage[3]) * B) / (B + S + p * B);
            let bonus_c = (1 + (Math.min(c + eff.critical[3], 1)) * D) / (1 + c * D);
            let bonus_D = (1 + c * (D + eff.criticalDamage[3])) / (1 + c * D);

            return {
                "bonus_S": {
                    value: bonus_S - 1
                },
                "bonus_p": {
                    value: bonus_p - 1
                },
                "bonus_c": {
                    value: bonus_c - 1
                },
                "bonus_D": {
                    value: bonus_D - 1
                },
            }
        },

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