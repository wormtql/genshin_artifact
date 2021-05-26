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
import SelectArtifactSet from "@c/SelectArtifactSet";
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

            filterLevel: deepCopy(DEFAULT_FILTER_LEVEL),
        }
    },
    methods: {
        getConstraint() {
            console.log(this.$data);
            return deepCopy(this.$data);
        },

        setConstraint(d) {
            if (!d) {
                // if a preset does not have constraint field, set to default
                this.constraintSet = deepCopy(DEFAULT_CONSTRAINT_SET);
                this.constraintMainTag = deepCopy(DEFAULT_CONSTRAINT_MAIN_TAG);
                this.filterLevel = deepCopy(DEFAULT_FILTER_LEVEL);
                return;
            }

            this.constraintSet = deepCopy(d.constraintSet);
            this.constraintMainTag = deepCopy(d.constraintMainTag);
            this.filterLevel = d.filterLevel ? deepCopy(d.filterLevel) : deepCopy(DEFAULT_FILTER_LEVEL);
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

<style scoped>
.cmt-label {
    width: 110px;
}

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
}

.plus {
    padding: 0 8px;
    color: #606266;
}

.confirm-button {
    width: 100%;
}
</style>