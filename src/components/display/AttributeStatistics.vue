<template>
    <div>
        <el-divider>梯度分析</el-divider>
        <!-- <el-alert
            type="warning"
            title="此处仅根据攻击力、暴击率、暴击伤害进行计算，仅供参考"
            :closable="false"
        ></el-alert> -->
        <div style="height: 300px">
            <v-chart :option="graphOption"></v-chart>
        </div>
        <p style="text-align: center">{{ analysisText }}</p>

        <div>
            <p
                class="item"
                v-show="howMuchBonusPerTag.bonus_S > 0"
            >每个攻击力词条提升：{{ x100(howMuchBonusPerTag["bonus_S"]) }}%</p>
            <p
                class="item"
                v-show="howMuchBonusPerTag.bonus_p > 0"
            >每个%攻击力词条提升：{{ x100(howMuchBonusPerTag["bonus_p"]) }}%</p>
            <p
                class="item"
                v-show="howMuchBonusPerTag.bonus_c > 0"
            >每个暴击率词条提升：{{ x100(howMuchBonusPerTag["bonus_c"]) }}%</p>
            <p
                class="item"
                v-show="howMuchBonusPerTag.bonus_D > 0"
            >每个暴击伤害词条提升：{{ x100(howMuchBonusPerTag["bonus_D"]) }}%</p>
             <p
                class="item"
                v-show="howMuchBonusPerTag.bonus_hpp > 0"
            >每个%生命值词条提升：{{ x100(howMuchBonusPerTag["bonus_hpp"]) }}%</p>
            <p
                class="item"
                v-show="howMuchBonusPerTag.bonus_hps > 0"
            >每个生命值词条提升：{{ x100(howMuchBonusPerTag["bonus_hps"]) }}%</p>
           
            <p
                class="item"
                v-show="howMuchBonusPerTag.bonus_em > 0"
            >每个元素精通词条提升：{{ x100(howMuchBonusPerTag["bonus_em"]) }}%</p>
            <p
                class="item"
                v-show="howMuchBonusPerTag.bonus_dfs > 0"
            >每个防御力词条提升：{{ x100(howMuchBonusPerTag["bonus_dfs"]) }}%</p>
            <p
                class="item"
                v-show="howMuchBonusPerTag.bonus_dfp > 0"
            >每%防御力词条提升输出：{{ x100(howMuchBonusPerTag["bonus_dfp"]) }}%</p>
            <p
                class="item"
                v-show="howMuchBonusPerTag.bonus_recharge > 0"
            >每个元素充能效率词条提升输出：{{ x100(howMuchBonusPerTag["bonus_recharge"]) }}%</p>
        </div>
    </div>
</template>

<script>
const bonusMap = {
    "bonus_S": "攻击力",
    "bonus_p": "%攻击力",
    "bonus_c": "暴击率",
    "bonus_D": "暴击伤害",
    "bonus_hpp": "%生命值",
    "bonus_hps": "生命值",
    "bonus_em": "元素精通",
    "bonus_dfp": "%防御力",
    "bonus_dfs": "防御力",
    "bonus_recharge": "元素充能效率",
}

export default {
    name: "AttributeStatistics",
    props: ["attribute", "howMuchBonusPerTag"],
    methods: {
        x100(value) {
            return (value * 100).toFixed(1);
        }
    },
    computed: {
        gradient() {
            let temp = [];
            for (const bonusName in bonusMap) {
                if (Object.prototype.hasOwnProperty.call(this.howMuchBonusPerTag, bonusName) && this.howMuchBonusPerTag[bonusName] > 0) {
                    temp.push([bonusMap[bonusName], this.howMuchBonusPerTag[bonusName]]);
                }
            }

            return temp;
        },

        analysisText() {
            if (this.gradient.length === 0) {
                return "任何词条都没有提升，或是出现了bug";
            }

            let maxIndex = 0;
            for (let i = 1; i < this.gradient.length; i++) {
                if (this.gradient[i][1] > this.gradient[maxIndex][1]) {
                    maxIndex = i;
                }
            }

            return `接下来提升最大的词条：${this.gradient[maxIndex][0]}`;
        },

        graphOption() {
            let data = [];
            // console.log(g);

            for (let entry of this.gradient) {
                data.push({
                    name: entry[0],
                    value: entry[1],
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