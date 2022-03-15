<template>
    <div>
        <div class="config-item">
            <h3
                class="config-title"
                style="margin-bottom: 20px"
            >限定圣遗物套装</h3>
            <div class="row">
                <el-radio
                    v-model="constraintSet.mode"
                    label="any"
                    class="radio"
                >任意</el-radio>
            </div>
            <div class="row">
                <el-radio
                    v-model="constraintSet.mode"
                    label="2"
                    class="radio"
                >
                    2
                </el-radio>
                <select-artifact-set
                    v-model="constraintSet.setName1"
                    :disabled="constraintSet.mode !== '2'"
                ></select-artifact-set>
            </div>
            <div class="row">
                <el-radio
                    v-model="constraintSet.mode"
                    label="22"
                    class="radio"
                >
                    2+2
                </el-radio>
                <select-artifact-set
                    v-model="constraintSet.setName2"
                    :disabled="constraintSet.mode !== '22'"
                ></select-artifact-set>
                <span class="plus">+</span>
                <select-artifact-set
                    v-model="constraintSet.setName3"
                    :disabled="constraintSet.mode !== '22'"
                ></select-artifact-set>
            </div>
            <div class="row" style="margin-bottom: 0">
                <el-radio
                    v-model="constraintSet.mode"
                    label="4"
                    class="radio"
                >
                    4
                </el-radio>
                <select-artifact-set
                    v-model="constraintSet.setName4"
                    :disabled="constraintSet.mode !== '4'"
                ></select-artifact-set>
            </div>
        </div>

        <div class="config-item">
            <h3 class="config-title" style="margin-bottom: 20px">限定主词条</h3>
            <div class="flex-row row">
                <span class="cmt-label fs-14 color-normal">时之沙</span>
                <select-art-main-tag-without-val
                    position="sand"
                    v-model="constraintMainTag.sand"
                ></select-art-main-tag-without-val>
            </div>
            <div class="flex-row row">
                <span class="cmt-label fs-14 color-normal">空之杯</span>
                <select-art-main-tag-without-val
                    position="cup"
                    v-model="constraintMainTag.cup"
                ></select-art-main-tag-without-val>
            </div>
            <div class="flex-row row" style="margin-bottom: 0">
                <span class="cmt-label fs-14 color-normal">理之冠</span>
                <select-art-main-tag-without-val
                    position="head"
                    v-model="constraintMainTag.head"
                ></select-art-main-tag-without-val>
            </div>
        </div>

        <div class="config-item">
            <h3 class="config-title" style="margin-bottom: 20px">限定属性最小值</h3>
            <div class="flex-row row">
                <span class="cmt-label fs-14 color-normal">生命值</span>
                <div>
                    <el-input size="small" v-model="constraintAttributeMin.life"></el-input>
                </div>
            </div>
            <div class="flex-row row">
                <span class="cmt-label fs-14 color-normal">攻击力</span>
                <div>
                    <el-input size="small" v-model="constraintAttributeMin.attack"></el-input>
                </div>
            </div>
            <div class="flex-row row">
                <span class="cmt-label fs-14 color-normal">防御力</span>
                <div>
                    <el-input size="small" v-model="constraintAttributeMin.defend"></el-input>
                </div>
            </div>
            <div class="flex-row row">
                <span class="cmt-label fs-14 color-normal">充能效率（%）</span>
                <div>
                    <el-input size="small" v-model="constraintAttributeMin.recharge"></el-input>
                </div>
            </div>
            <div class="flex-row row">
                <span class="cmt-label fs-14 color-normal">元素精通</span>
                <div>
                    <el-input size="small" v-model="constraintAttributeMin.elementalMastery"></el-input>
                </div>
            </div>
            <div class="flex-row row">
                <span class="cmt-label fs-14 color-normal">暴击率（%）</span>
                <div>
                    <el-input size="small" v-model="constraintAttributeMin.critical"></el-input>
                </div>
            </div>
            <div class="flex-row row" style="margin-bottom: 0">
                <span class="cmt-label fs-14 color-normal">暴击伤害（%）</span>
                <div>
                    <el-input size="small" v-model="constraintAttributeMin.criticalDamage"></el-input>
                </div>
            </div>
        </div>

        <div class="config-item">
            <h3 class="config-title">过滤等级</h3>
            <el-slider
                v-model="levelDelegate"
                range
                :min="0"
                :max="20"
                :marks="marks"
                :show-tooltip="false"
            ></el-slider>
        </div>
    </div>
</template>

<script>
import SelectArtifactSet from "@c/select/SelectArtifactSet";
import SelectArtMainTagWithoutVal from '@c/SelectArtMainTagWithoutVal.vue';

import deepCopy from "@util/deepcopy";

const DEFAULT_CONSTRAINT_SET = {
    mode: "any",
    setName1: "berserker",
    setName2: "berserker",
    setName3: "berserker",
    setName4: "berserker",
};

const DEFAULT_CONSTRAINT_MAIN_TAG = {
    sand: "any",
    cup: "any",
    head: "any",
}

const DEFAULT_FILTER_LEVEL = {
    min: 16,
    max: 20,
}

const DEFAULT_CONSTRAINT_ATTRIBUTE_MIN = {
    life: "0",
    attack: "0",
    defend: "0",
    recharge: "100",
    elementalMastery: "0",
    critical: "0",
    criticalDamage: "50",
}

export default {
    name: "Config",
    components: {
        SelectArtifactSet,
        SelectArtMainTagWithoutVal,
    },
    created() {
        let marks = {};
        for (let i = 0; i <= 20; i++) {
            marks[i] = i.toString();
        }
        this.marks = marks;
    },
    data() {
        return {
            constraintSet: deepCopy(DEFAULT_CONSTRAINT_SET),
            constraintMainTag: deepCopy(DEFAULT_CONSTRAINT_MAIN_TAG),
            constraintAttributeMin: deepCopy(DEFAULT_CONSTRAINT_ATTRIBUTE_MIN),
            filterLevel: deepCopy(DEFAULT_FILTER_LEVEL),
        }
    },
    methods: {
        getConstraint() {
            return {
                constraintSet: deepCopy(this.constraintSet),
                constraintMainTag: deepCopy(this.constraintMainTag),
                constraintAttributeMin: this.getConstraintAttributeMin(),
                filterLevel: deepCopy(this.filterLevel),
            };
        },

        getConstraintAttributeMin() {
            let cam = this.constraintAttributeMin;
            return {
                attack: parseFloat(cam.attack) ?? 0,
                life: parseFloat(cam.life) ?? 0,
                defend: parseFloat(cam.defend) ?? 0,
                recharge: (parseFloat(cam.recharge) ?? 100) / 100,
                elementalMastery: parseFloat(cam.elementalMastery) ?? 0,
                critical: (parseFloat(cam.critical) ?? 0) / 100,
                criticalDamage: (parseFloat(cam.criticalDamage) ?? 50) / 100,
            }
        },

        setConstraint(d) {
            if (!d) {
                // if a preset does not have constraint field, set to default
                this.constraintSet = deepCopy(DEFAULT_CONSTRAINT_SET);
                this.constraintMainTag = deepCopy(DEFAULT_CONSTRAINT_MAIN_TAG);
                this.filterLevel = deepCopy(DEFAULT_FILTER_LEVEL);
                this.constraintAttributeMin = deepCopy(DEFAULT_CONSTRAINT_ATTRIBUTE_MIN);
                return;
            }

            this.constraintSet = deepCopy(d.constraintSet);
            this.constraintMainTag = deepCopy(d.constraintMainTag);
            this.filterLevel = d.filterLevel ? deepCopy(d.filterLevel) : deepCopy(DEFAULT_FILTER_LEVEL);
            let cam = d.constraintAttributeMin;
            if (cam) {
                this.constraintAttributeMin = {
                    life: cam.life.toString(),
                    attack: cam.attack.toString(),
                    defend: cam.defend.toString(),
                    recharge: (cam.recharge * 100).toString(),
                    elementalMastery: cam.elementalMastery.toString(),
                    critical: (cam.critical * 100).toString(),
                    criticalDamage: (cam.criticalDamage * 100).toString(),
                };
            } else {
                this.constraintAttributeMin = deepCopy(DEFAULT_CONSTRAINT_ATTRIBUTE_MIN);
            }
            
        }
    },
    computed: {
        levelDelegate: {
            get() {
                return [this.filterLevel.min, this.filterLevel.max];
            },

            set(value) {
                this.filterLevel.min = value[0];
                this.filterLevel.max = value[1];
            }
        }
    }
}
</script>

<style lang="scss" scoped>
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

.radio {
    width: 80px;
}

.row {
    margin-bottom: 18px;

    .cmt-label {
        width: 110px;
    }
}

.plus {
    padding: 0 8px;
    color: #606266;
}

.confirm-button {
    width: 100%;
}
</style>