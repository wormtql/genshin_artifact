<template>
    <div>
        <div v-if="calculating">
            计算中
        </div>
        <div class="main-div" v-else-if="!calculating && !resultData.error">
            <div class="left">
                <el-alert
                    title="由于原神战斗体系比较复杂，buff体系繁多，因此计算结果仅供参考"
                    :closable="false"
                >
                </el-alert>

                <h3 class="title">最佳搭配</h3>
                <div class="artifact-div">
                    <artifact-display
                        v-for="art in artifacts"
                        :key="art.id"
                        :buttons="true"
                        :item="art"
                        class="artifact"
                        @toggle="toggle(art.id)"
                    ></artifact-display>
                </div>
                <div>
                    <el-button size="small" @click="disableArtifacts">禁用以上圣遗物</el-button>
                </div>

                <h3 class="title">最大值</h3>
                <el-alert
                    title="不同目标函数的最大值不可相互比较；输出类型的最大值也并不是最终期望伤害，因此仅供参考"
                    type="warning"
                    :closable="false"
                    style="margin-bottom: 12px"
                ></el-alert>
                <p class="max-value">{{ resultData.value.toFixed(3) }}</p>
            </div>
            <div class="right">
                <attribute-panel :panel="resultData.attribute" style="width: 300px" v-if="!calculating"></attribute-panel>
            </div>
        </div>
        <div v-else-if="!calculating && resultData.error">
            没有合适的圣遗物
        </div>
    </div>
</template>

<script>
import ArtifactDisplay from "@c/ArtifactDisplay";
import AttributePanel from "@c/AttributePanel";

import compute from "@alg/attribute_target/compute_artifacts_promise";
import timer from "@util/timer";

import targetFunctionsData from "@asset/target_functions/data";

export default {
    name: "ResultPage",
    components: {
        ArtifactDisplay,
        AttributePanel,
    },
    props: {
    },
    data: function () {
        return {
            resultData: {
                error: false,
                value: 0,
                artifacts: [],
                attribute: {},
            },

            calculating: true,
        }
    },
    methods: {
        toggle(id) {
            this.$store.commit("artifacts/toggleById", { id });
        },

        disableArtifacts() {
            for (let id of this.artifactsId) {
                this.$store.commit("artifacts/disableArtifactById", { id });
            }
            // this.$message("操作成功");
        },

        convertTArgs(name, args) {
            let data = targetFunctionsData[name];
            if (data && data.config && data.config.compact) {
                return data.config.compact(args);
            }
            return args;
        },

        doCompute() {
            let character = this.$parent.characterInfo;
            let weapon = this.$parent.weaponInfo;
            let artifacts = this.$parent.getArtifacts();
            let constraintConfig = this.$parent.selected.constraintConfig;
            let targetFuncName = this.$parent.selected.targetFuncName;
            let targetFuncArgs = this.convertTArgs(targetFuncName, this.$parent.selected.targetFuncArgs);
            // console.log(weapon);

            let loading = this.$loading({
                lock: true,
                text: "莫娜占卜中",
            });
            this.calculating = true;

            // this is a web worker wrapped by a promise
            let promise = compute(artifacts, character, weapon, targetFuncName, targetFuncArgs, constraintConfig).then(result => {
                this.resultData = {
                    artifacts: Object.values(result.combo),
                    value: result.value,
                    attribute: result.attribute,
                    error: result.error,
                };
                loading.close();
                this.calculating = false;
            }).catch(reason => {
                this.$message.error("计算过程发生错误：" + reason);
            });

            // record time
            timer(promise).then(time => {
                console.log(`complete after ${time}ms`);
            });
        }
    },
    computed: {
        filteredArtifacts() {
            return this.resultData.artifacts.filter(item => !!item);
        },

        artifactsId() {
            return this.filteredArtifacts.map(item => item.id);
        },

        artifacts() {
            let temp = [];
            let map = this.$store.getters["artifacts/artifactsById"];

            for (let id of this.artifactsId) {
                temp.push(map[id]);
            }

            return temp;
        }
    }
}
</script>

<style scoped>
.title {
    /* background:rgb(74, 99, 211); */
    padding: 0px 16px;
    display: inline-block;
    /* color: white; */
    border-radius: 3px;
    font-size: 14px;
    border-bottom: 5px solid rgb(74, 99, 211);
    color: #555555;
}

.artifact-div {
    display: flex;
    flex-wrap: wrap;
}

.artifact {
    margin: 0 16px 16px 0;
}

.max-value {
    padding: 0;
    margin: 0;
    font-size: 14px;
}

.left {
    flex: 1;
    margin-right: 16px;
}

.main-div {
    display: flex;
}
</style>