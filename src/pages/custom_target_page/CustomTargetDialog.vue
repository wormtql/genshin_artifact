<template>
    <el-dialog :visible.sync="show" title="自定义单元素期望伤害函数" width="80%" :before-close="onClose">
        <h2>名称</h2>
        <el-input v-model="name" style="margin-bottom: 8px"></el-input>
        <el-tag type="danger" v-show="!checkName[0]">{{ checkName[1] || "" }}</el-tag>

        <h2>元素类型</h2>
        <el-radio-group v-model="element">
            <el-radio-button label="fire">火</el-radio-button>
            <el-radio-button label="thunder">雷</el-radio-button>
            <el-radio-button label="ice">冰</el-radio-button>
            <el-radio-button label="water">水</el-radio-button>
            <el-radio-button label="rock">岩</el-radio-button>
            <el-radio-button label="wind">风</el-radio-button>
            <el-radio-button label="physical">物理</el-radio-button>
        </el-radio-group>

        <h2>普通攻击</h2>
        <el-row type="flex" align="middle">
            <el-col :span="4"><span>频率</span></el-col>
            <el-col :span="20">
                <el-slider :min="0" :max="1" :step="0.01" v-model="aFreq" show-input></el-slider>
            </el-col>
        </el-row>
        <el-row type="flex" align="middle" v-show="!isPhysical">
            <el-col :span="4"><span>元素伤害权重</span></el-col>
            <el-col :span="20">
                <el-slider :min="0" :max="1" :step="0.01" v-model="aRatio" show-input></el-slider>
            </el-col>
        </el-row>
        <el-row type="flex" align="middle">
            <el-col :span="4"><span>倍率</span></el-col>
            <el-col :span="20">
                <el-slider :min="0" :max="10" :step="0.1" v-model="aTimes" show-input></el-slider>
            </el-col>
        </el-row>

        <h2>重击</h2>
        <el-row type="flex" align="middle">
            <el-col :span="4"><span>频率</span></el-col>
            <el-col :span="20">
                <el-slider :min="0" :max="1" :step="0.01" v-model="bFreq" show-input></el-slider>
            </el-col>
        </el-row>
        <el-row type="flex" align="middle" v-show="!isPhysical">
            <el-col :span="4"><span>元素伤害权重</span></el-col>
            <el-col :span="20">
                <el-slider :min="0" :max="1" :step="0.01" v-model="bRatio" show-input></el-slider>
            </el-col>
        </el-row>
        <el-row type="flex" align="middle">
            <el-col :span="4"><span>倍率</span></el-col>
            <el-col :span="20">
                <el-slider :min="0" :max="10" :step="0.1" v-model="bTimes" show-input></el-slider>
            </el-col>
        </el-row>

        <div v-show="!isPhysical">
            <h2>元素战技</h2>
            <el-row type="flex" align="middle">
                <el-col :span="4"><span>频率</span></el-col>
                <el-col :span="20">
                    <el-slider :min="0" :max="1" :step="0.01" v-model="eFreq" show-input></el-slider>
                </el-col>
            </el-row>
            <!-- <el-row type="flex" align="middle">
                <el-col :span="4"><span>元素伤害权重</span></el-col>
                <el-col :span="20">
                    <el-slider :min="0" :max="1" :step="0.01" v-model="eRatio" show-input></el-slider>
                </el-col>
            </el-row> -->
            <el-row type="flex" align="middle">
                <el-col :span="4"><span>倍率</span></el-col>
                <el-col :span="20">
                    <el-slider :min="0" :max="10" :step="0.1" v-model="eTimes" show-input></el-slider>
                </el-col>
            </el-row>
        </div>

        <div v-show="!isPhysical">
            <h2>元素爆发</h2>
            <el-row type="flex" align="middle">
                <el-col :span="4"><span>频率</span></el-col>
                <el-col :span="20">
                    <el-slider :min="0" :max="1" :step="0.01" v-model="qFreq" show-input></el-slider>
                </el-col>
            </el-row>
            <!-- <el-row type="flex" align="middle">
                <el-col :span="4"><span>元素伤害权重</span></el-col>
                <el-col :span="20">
                    <el-slider :min="0" :max="1" :step="0.01" v-model="qRatio" show-input></el-slider>
                </el-col>
            </el-row> -->
            <el-row type="flex" align="middle">
                <el-col :span="4"><span>倍率</span></el-col>
                <el-col :span="20">
                    <el-slider :min="0" :max="10" :step="0.1" v-model="qTimes" show-input></el-slider>
                </el-col>
            </el-row>
        </div>
        

        <template #footer>
            <el-button @click="onClose">取消</el-button>
            <el-button type="primary" @click="onConfirm" :disabled="!checkAll[0]">确定</el-button>
        </template>
    </el-dialog>
</template>

<script>
export default {
    name: "CustomTargetDialog",
    props: {
        show: {
            type: Boolean
        },
        usedNames: {
            type: Array,
            default: () => [],
        }
    },
    methods: {
        onClose() {
            this.$emit("close");
        },
        onConfirm() {
            this.$emit("confirm", this.compact());
        },
        compact() {
            return {
                name: this.name,
                config: {
                    element: this.element,

                    aFreq: Number(this.aFreq),
                    aRatio: this.isPhysical ? 0 : Number(this.aRatio),
                    aTimes: Number(this.aTimes),

                    bFreq: Number(this.bFreq),
                    bRatio: this.isPhysical ? 0 : Number(this.bRatio),
                    bTimes: Number(this.bTimes),

                    eFreq: this.isPhysical ? 0 : Number(this.eFreq),
                    eTimes: Number(this.eTimes),

                    qFreq: this.isPhysical ? 0 : Number(this.qFreq),
                    qTimes: Number(this.qTimes),
                }
            }
        }
    },
    data: function() {
        return {
            name: "",

            element: "fire",

            aFreq: 0,
            aRatio: 0,
            aTimes: 0,

            bFreq: 0,
            bRatio: 0,
            bTimes: 0,

            eFreq: 0,
            // eRatio: 1,
            eTimes: 0,

            qFreq: 0,
            // qRatio: 1,
            qTimes: 0
        }
    },
    computed: {
        isPhysical() {
            return this.element === "physical";
        },

        checkName() {
            if (this.name === "") {
                return [false, "名称不能为空"];
            }
            // window.console.log(this.usedNames);
            if (this.usedNames.indexOf(this.name) !== -1) {
                return [false, "名称已使用"];
            }

            return [true];
        },

        checkAll() {
            if (!this.checkName[0]) {
                return this.checkName;
            }

            return [true];
        }
    }
}
</script>