<template>
    <div>
        <el-table
            :data="tableData"
            size="mini"
        >
            <el-table-column
                label="词条"
                prop="chs"
            ></el-table-column>
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
        </el-table>
    </div>
</template>

<script>
import { howManyUpgradeCount } from "@util/artifacts";
import { secondaryTags } from "@asset/tags";

export default {
    name: "ArtifactsSetStatistics",
    props: ["artifacts"],
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

        tableData() {
            let temp = [];
            for (let tagName in this.upgradeCount) {
                let tagData = secondaryTags[tagName];
                temp.push({
                    chs: tagData.chs,
                    value: this.upgradeCount[tagName],
                });
            }
            return temp;
        }
    }
}
</script>