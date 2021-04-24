<template>
    <div>
        <el-breadcrumb>
            <el-breadcrumb-item>圣遗物统计</el-breadcrumb-item>
        </el-breadcrumb>
        <el-divider></el-divider>

        <el-row style="margin-bottom: 16px">
            <el-col
                :span="24"
            >
                <el-card>
                    <div slot="header">
                        副词条效率分布（只计入20级5星圣遗物）
                    </div>

                    <!-- <div
                        class="filter-bar"
                    >
                        <div class="filter-item">
                            <el-switch
                                active-text="只看5星"
                                v-model="subStatEff5StarOnly"
                            ></el-switch>
                        </div>
                        <div class="flex-row filter-item">
                            <span class="fs-12">等级大于等于</span>
                            <el-input-number
                                v-model="subStatEffLevelMin"
                                :min="0"
                                :max="subStatEffLevelMax"
                                size="mini"
                            ></el-input-number>
                        </div>
                        <div class="flex-row filter-item">
                            <span class="fs-12">等级小于等于</span>
                            <el-input-number
                                v-model="subStatEffLevelMax"
                                :min="subStatEffLevelMin"
                                :max="20"
                                size="mini"
                            ></el-input-number>
                        </div>
                        
                    </div> -->

                    <div style="height: 400px">
                        <v-chart :option="subStatEffDistribution.options"></v-chart>
                    </div>

                    <el-tag>
                        圣遗物综合分数：{{ overallRating.toFixed(3) }}
                    </el-tag>
                </el-card>
            </el-col>
        </el-row>

        <el-row :gutter="16" style="margin-bottom: 16px">
            <el-col :span="12">
                <el-card>
                    <div slot="header">
                        星级分布
                    </div>
                    <div style="height: 400px">
                        <v-chart :option="starDistribution"></v-chart>
                    </div>
                </el-card>
            </el-col>
            <el-col :span="12">
                <el-card>
                    <div slot="header">
                        等级分布
                    </div>
                    <div style="height: 400px">
                        <v-chart :option="levelDistribution"></v-chart>
                    </div>
                </el-card>
            </el-col>
        </el-row>

        <el-row :gutter="16">
            <el-col :span="8">
                <el-card>
                    <div slot="header">
                        时之沙主词条分布
                        <div style="float: right">
                            <el-switch
                                v-model="sandMainStat16Only"
                                active-text=">=16级"
                            ></el-switch>
                        </div>
                    </div>
                    <div style="height: 300px">
                        <v-chart :option="sandMainStatDistribution"></v-chart>
                    </div>
                </el-card>
            </el-col>
            <el-col :span="8">
                <el-card>
                    <div slot="header">
                        空之杯主词条分布
                        <div style="float: right">
                            <el-switch
                                v-model="cupMainStat16Only"
                                active-text=">=16级"
                            ></el-switch>
                        </div>
                    </div>
                    <div style="height: 300px">
                        <v-chart :option="cupMainStatDistribution"></v-chart>
                    </div>
                </el-card>
            </el-col>
            <el-col :span="8">
                <el-card>
                    <div slot="header">
                        理之冠主词条分布
                        <div style="float: right">
                            <el-switch
                                v-model="headMainStat16Only"
                                active-text=">=16级"
                            ></el-switch>
                        </div>
                    </div>
                    <div style="height: 300px">
                        <v-chart :option="headMainStatDistribution"></v-chart>
                    </div>
                </el-card>
            </el-col>
        </el-row>
    </div>
</template>

<script>
import { mapGetters } from "vuex";

import starDistribution from "./distributions/starDistribution";
import levelDistribution from "./distributions/levelDistribution";
import sandMainStatDistribution from "./distributions/sandMainStatDistribution";
import cupMainStatDistribution from "./distributions/cupMainStatDistribution";
import headMainStatDistribution from "./distributions/headMainStatDistribution";
import subStatEffDistribution from "./distributions/SubStatEffDistribution";

export default {
    name: "ArtifactsStatisticsPage",
    data() {
        return {
            sandMainStat16Only: false,
            cupMainStat16Only: false,
            headMainStat16Only: false,

            // subStatEffLevelMin: 0,
            // subStatEffLevelMax: 20,
            // subStatEff5StarOnly: true,
        }
    },
    computed: {
        ...mapGetters("artifacts", ["allArtifacts"]),
        starDistribution() {
            return starDistribution(this);
        },
        levelDistribution() {
            return levelDistribution(this);
        },
        sandMainStatDistribution() {
            return sandMainStatDistribution(this);
        },
        cupMainStatDistribution() {
            return cupMainStatDistribution(this);
        },
        headMainStatDistribution() {
            return headMainStatDistribution(this);
        },
        subStatEffDistribution() {
            return subStatEffDistribution(this);
        },

        overallRating() {
            let eff = this.subStatEffDistribution.eff;

            let weight = {
                critical: 1,
                lifePercentage: 0.1,
                attackPercentage: 1,
                recharge: 0.1,
                defendPercentage: 0.08,
                criticalDamage: 1,
                attackStatic: 0.2,
                defendStatic: 0.05,
                elementalMastery: 0.5,
                lifeStatic: 0,
            };

            let sum = Object.values(eff).reduce((a, b) => a + b);

            let score = 0;
            for (let i in weight) {
                score += weight[i] * eff[i];
            }
            
            return 100 * score / sum;
        }
    }
}
</script>

<style lang="scss" scoped>
.filter-bar {
    margin-bottom: 16px;
    background: #ecf5ff;
    border: 1px solid #d9ecff;
    border-radius: 3px;
    padding: 8px;

    .filter-item {
        margin-bottom: 12px;

        &:last-of-type {
            margin: 0;
        }
    }
}
</style>