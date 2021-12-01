<template>
    <div>
        <div class="filter">
            <filter-potential-result
                ref="filterConfig"
                v-model="displayFilterConfig"
            ></filter-potential-result>
        </div>
        <div class="container">
            <artifact-display
                v-for="(item, index) in resultToDisplay"
                :key="index"
                :item="item[0]"
                :extra="'分数：' + item[1].toFixed(3)"
                width="240px"
                class="art"
            ></artifact-display>

            <el-alert
                v-show="finalResult.length === 0"
                title="没有满足条件的圣遗物"
                :closable="false"
                type="warning"
            ></el-alert>
        </div>

        <div
            class="pager"
        >
            <el-pagination
                layout="prev, pager, next"
                :total="resultCount"
                background
                :current-page.sync="currentPage"
                hide-on-single-page
                :page-size="pageSize"
            ></el-pagination>
        </div>
    </div>
</template>

<script>
import { computeAll } from "@alg/potential/compute_artifact_potential_promise";

import ArtifactDisplay from "@c/ArtifactDisplay";
import FilterPotentialResult from "@c/filter/FilterPotentialResult";


const PAGE_SIZE = 10;

export default {
    name: "ResultOfAll",
    components: {
        ArtifactDisplay,
        FilterPotentialResult,
    },
    data: function () {
        return {
            result: [],

            currentPage: 1,

            pageSize: PAGE_SIZE,

            displayFilterConfig: {
                filterSlots: ["flower", "feather", "sand", "cup", "head"],
                minLevel: 0,
                maxLevel: 20,
                filterSetName: "any",
                filterMainTag: "any",
            }
        }
    },
    computed: {
        finalResult() {
            let temp = this.result.slice();

            temp = temp.filter(item => this.filter(item[0]));

            temp.sort((a, b) => b[1] - a[1]);

            return temp;
        },

        resultToDisplay() {
            let offset = (this.currentPage - 1) * PAGE_SIZE;
            let end = offset + PAGE_SIZE;
            return this.finalResult.slice(offset, end);
        },

        resultCount() {
            return this.finalResult.length;
        },
    },
    methods: {
        compute({ potentialFunction }) {
            let loading = this.$loading({
                lock: true,
                text: "莫娜占卜中",
            });
            let arts = this.$store.getters["artifacts/allFlat"];

            let fil = item => (item.star ?? 5) >= 4;
            let filteredArts = arts.filter(fil);

            let name = potentialFunction.name;
            let pArgs = potentialFunction.args;

            computeAll(filteredArts, name, pArgs).then(result => {
                this.result = result.filter(items => items[1] > 0);
                loading.close();
                // console.log(result);
            });
        },

        filter(artifact) {
            let c = this.displayFilterConfig;

            let pos = artifact.position;
            let f1 = c.filterSlots.indexOf(pos) !== -1;

            let level = artifact.level ?? 20;
            let f2 = level >= c.minLevel && level <= c.maxLevel;

            let f3 = c.filterSetName === "any" || c.filterSetName === artifact.setName;

            let mainTagName = artifact.mainTag.name;
            let f4 = c.filterMainTag === "any" || c.filterMainTag === mainTagName;

            return f1 && f2 && f3 && f4;
        }
    }
}
</script>

<style scoped>
.container {
    display: flex;
    flex-wrap: wrap;
}

.art {
    margin: 0 16px 16px 0;
}

.filter {
    margin-bottom: 24px;
}

.pager {
    display: flex;
    align-items: center;
    justify-content: center;
    margin-top: 24px;
}
</style>
