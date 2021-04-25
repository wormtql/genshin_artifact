<template>
    <div>
        <div class="filter">
            <el-checkbox v-model="filNotFull">未满级圣遗物</el-checkbox>
            <el-checkbox v-model="filFull">满级圣遗物</el-checkbox>
        </div>
        <div class="container">
            <artifact-display
                v-for="(item, index) in finalResult"
                :key="index"
                :item="item[0]"
                :extra="'分数：' + item[1].toFixed(3)"
                class="art"
            ></artifact-display>

            <el-alert
                v-show="finalResult.length === 0"
                title="没有满足条件的圣遗物"
                :closable="false"
            ></el-alert>
        </div>
    </div>
</template>

<script>
import { computeAll } from "@alg/potential/compute_artifact_potential_promise";

import ArtifactDisplay from "@c/ArtifactDisplay";

export default {
    name: "ResultOfAll",
    components: {
        ArtifactDisplay
    },
    data: function () {
        return {
            result: [],
            // filters: [],

            filNotFull: true,
            filFull: true,

            // loading: true,
        }
    },
    computed: {
        finalResult() {
            let temp = this.result.slice();

            for (let fil of this.filters) {
                temp = temp.filter(item => fil(item[0]))
            }

            temp.sort((a, b) => b[1] - a[1]);

            return temp;
        },

        filters() {
            let temp = [];
            let getLevel = art => {
                if (Object.prototype.hasOwnProperty.call(art, "level")) {
                    return art.level;
                }
                return 20;
            };
            if (!this.filNotFull) {
                temp.push(item => getLevel(item) === 20);
            }
            if (!this.filFull) {
                temp.push(item => getLevel(item) < 20);
            }

            return temp;
        }
    },
    methods: {
        compute() {
            let loading = this.$loading({
                lock: true,
                text: "莫娜计算中",
            });
            let arts = this.$store.getters["artifacts/allFlat"];

            let fil = item => (item.star ?? 5) >= 4;
            let filteredArts = arts.filter(fil);
            // console.log(filteredArts);

            let name = this.$parent.$data.selected.funcName;
            let pArgs = this.$parent.$data.selected.pArgs;

            computeAll(filteredArts, name, pArgs).then(result => {
                this.result = result;
                loading.close();
                // console.log(result);
            });
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
</style>