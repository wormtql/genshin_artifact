<template>
    <div>
        <el-divider>有效词条分布</el-divider>
        <artifacts-tag-eff-graph
            :artifacts="artifacts"
            style="margin-bottom: 32px"
        ></artifacts-tag-eff-graph>

        <el-divider>明细</el-divider>
        <el-table
            :data="tableData"
            size="mini"
        >
            <el-table-column
                label="词条"
                prop="chs"
            ></el-table-column>
            <el-table-column
                label="值"
            >
                <template slot-scope="scope">
                    {{ scope.row.number }}
                </template>
            </el-table-column>
            <el-table-column
                label="最低强化数"
            >
                <template slot-scope="scope">
                    {{ scope.row.value[0] }}
                </template>
            </el-table-column>
            <el-table-column
                label="最高强化数"
            >
                <template slot-scope="scope">
                    {{ scope.row.value[1] }}
                </template>
            </el-table-column>
            <el-table-column
                label="有效词条数"
            >
                <template slot-scope="scope">
                    {{ scope.row.eff.toFixed(3) }}
                </template>
            </el-table-column>
        </el-table>
        <!-- <p class="single-item">共强化次数（理论最大值：45）：{{ totalUpgradeCount }}</p> -->
        <p class="single-item">总有效词条数（理论最大值：45）：{{ totalEff.toFixed(3) }}</p>
        <!-- <p class="single-item">总有效词条数（理论最大值：45）：{{ totalEff.toFixed(3) }}</p> -->
        <p class="single-item">暴击率+暴击伤害有效词条数：{{ validEff(["critical", "criticalDamage"]).toFixed(3) }}</p>
        <p class="single-item">暴击率+暴击伤害+%攻击力有效词条数：{{ validEff(["critical", "criticalDamage", "attackPercentage"]).toFixed(3) }}</p>
    </div>
</template>

<script>
import { howManyUpgradeCount, getValueEff } from "@util/artifacts";
import { secondaryTags } from "@asset/tags";

import ArtifactsTagEffGraph from "./ArtifactsTagEffGraph";

export default {
    name: "ArtifactsSetStatistics",
    components: {
        ArtifactsTagEffGraph,
    },
    props: ["artifacts"],
    methods: {
        validEff(validTagNames) {
            let temp = 0;
            for (let name of validTagNames) {
                temp += this.tagEff[name] ?? 0;
            }
            return temp;
        }
    },
    computed: {
        upgradeCount() {
            let temp = {};

            if (!this.artifacts) {
                return {};
            }

            for (let artifact of this.artifacts) {
                let star = artifact.star ?? 5;
                if (star <= 3) {
                    continue;
                }
                for (let tag of artifact.normalTags) {
                    let name = tag.name;
                    let value = tag.value;
                    let [min, max] = howManyUpgradeCount(value, name, star);
                    if (temp[name]) {
                        temp[name][0] += min;
                        temp[name][1] += max;
                    } else {
                        temp[name] = [min, max];
                    }
                }
            }

            return temp;
        },

        totalUpgradeCount() {
            return Object.values(this.upgradeCount).reduce((a, b) => [a[0] + b[0], a[1] + b[1]]);
        },

        upgradeValue() {
            let temp = {};

            if (!this.artifacts) {
                return {};
            }

            for (let artifact of this.artifacts) {
                let star = artifact.star ?? 5;
                if (star <= 3) {
                    continue;
                }
                for (let tag of artifact.normalTags) {
                    let name = tag.name;
                    let value = tag.value;
                    if (temp[name]) {
                        temp[name] += value;
                    } else {
                        temp[name] = value;
                    }
                }
            }

            for (let name in temp) {
                if (secondaryTags[name].percentage) {
                    temp[name] = `${(temp[name] * 100).toFixed(1)}%`;
                } else {
                    temp[name] = temp[name].toString();
                }
            }

            return temp;
        },

        totalEff() {
            return Object.values(this.tagEff).reduce((a, b) => a + b);
        },

        tagEff() {
            let temp = {};
            if (!this.artifacts) {
                return {};
            }

            for (let artifact of this.artifacts) {
                let star = artifact.star ?? 5;
                if (star <= 3) {
                    continue;
                }
                for (let tag of artifact.normalTags) {
                    let name = tag.name;
                    let value = tag.value;
                    let eff = getValueEff(value, name, star);
                    if (temp[name]) {
                        temp[name] += eff;
                    } else {
                        temp[name] = eff;
                    }
                }
            }

            return temp;
        },

        tableData() {
            let temp = [];
            for (let tagName in this.upgradeCount) {
                let tagData = secondaryTags[tagName];
                temp.push({
                    chs: tagData.chs,
                    value: this.upgradeCount[tagName],
                    eff: this.tagEff[tagName],
                    number: this.upgradeValue[tagName],
                });
            }
            return temp;
        }
    }
}
</script>