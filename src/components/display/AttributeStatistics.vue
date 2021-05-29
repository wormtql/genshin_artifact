<template>
    <div>
        <el-divider>梯度分析</el-divider>
        <div style="height: 300px">
            <v-chart :option="graphOption"></v-chart>
        </div>
        <p style="text-align: center">{{ analysisText }}</p>
    </div>
</template>

<script>
export default {
    name: "AttributeStatistics",
    props: ["attribute"],
    methods: {
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
            let gCritical = (B + S + B * p) * D;
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
            console.log(g);

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