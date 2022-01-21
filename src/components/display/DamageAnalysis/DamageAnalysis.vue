<template>
    <div>
        <div style="margin-bottom: 16px;">
            <el-radio-group v-model="damageType" size="small" style="margin-right: 24px;">
                <el-radio-button label="normal">{{ normalDamageName }}</el-radio-button>
                <el-radio-button v-if="showMeltOption" label="melt">融化</el-radio-button>
                <el-radio-button v-if="showVaporizeOption" label="vaporize">蒸发</el-radio-button>
            </el-radio-group>

            <span class="damage-display" v-if="damageType === 'normal'">{{ damageNormal }}</span>
            <span class="damage-display" v-if="damageType === 'melt'">{{ damageMelt }}</span>
            <span class="damage-display" v-if="damageType === 'vaporize'">{{ damageVaporize }}</span>
        </div>

        <div class="header-row">
            <div>
                <div class="big-title base-damage-region">基础伤害乘区</div>
                <div class="header-row">
                    <damage-analysis-util
                        v-if="atkRatioState.length > 0"
                        :arr="atkState"
                        title="攻击力"
                    ></damage-analysis-util>
                    <damage-analysis-util
                        v-if="atkRatioState.length > 0"
                        :arr="atkRatioState"
                        title="攻击力倍率"
                    ></damage-analysis-util>
                    <damage-analysis-util
                        v-if="defRatioState.length > 0"
                        :arr="defState"
                        title="防御力"
                    ></damage-analysis-util>
                    <damage-analysis-util
                        v-if="defRatioState.length > 0"
                        :arr="defRatioState"
                        title="防御力倍率"
                    ></damage-analysis-util>
                    <damage-analysis-util
                        v-if="hpRatioState.length > 0"
                        :arr="hpState"
                        title="生命值"
                    ></damage-analysis-util>
                    <damage-analysis-util
                        v-if="hpRatioState.length > 0"
                        :arr="hpRatioState"
                        title="生命值倍率"
                    ></damage-analysis-util>
                    <damage-analysis-util
                        v-if="extraDamageState.length > 0"
                        :arr="extraDamageState"
                        title="其他伤害"
                    ></damage-analysis-util>
                </div>
            </div>
            <div>
                <div class="big-title critical-region">暴击乘区</div>
                <div class="header-row">
                    <damage-analysis-util
                        :arr="criticalState"
                        title="暴击率"
                    ></damage-analysis-util>
                    <damage-analysis-util
                        :arr="criticalDamageState"
                        title="暴击伤害"
                    ></damage-analysis-util>
                </div>
            </div>
            <div>
                <div class="big-title bonus-region">增伤乘区</div>
                <div class="header-row">
                    <damage-analysis-util
                        :arr="bonusState"
                        title="伤害加成"
                    ></damage-analysis-util>
                </div>
            </div>
            <div v-show="damageType === 'melt' || damageType === 'vaporize'">
                <div class="big-title reaction-ratio-region">反应倍率</div>
                <div class="header-row" style="height: 100%; display: flex; align-items: center; justify-content: center">
                    <span>{{ reactionRatio }}</span>
                </div>
            </div>
            <div v-if="damageType === 'melt'">
                <div class="big-title melt-region">增幅伤害加成</div>
                <div class="header-row">
                    <damage-analysis-util
                        :arr="meltEnhanceState"
                        title="融化伤害加成"
                    ></damage-analysis-util>
                </div>
            </div>
            <div v-if="damageType === 'vaporize'">
                <div class="big-title vaporize-region">增幅伤害加成</div>
                <div class="header-row">
                    <damage-analysis-util
                        :arr="meltEnhanceState"
                        title="蒸发伤害加成"
                    ></damage-analysis-util>
                </div>
            </div>
        </div>
    </div>
</template>

<script>
import DamageAnalysisUtil from "./DamageAnalysisUtil"

function sum(arr) {
    let s = 0
    for (let item of arr) {
        if (item.checked) {
            s += parseFloat(item.value)
        }
    }
    return s
}

export default {
    name: "DamageAnalysis",
    components: {
        DamageAnalysisUtil
    },
    data() {
        return {
            damageType: "normal",
            element: "Pyro",

            atkState: [{ name: "test", value: 1000, checked: true }],
            atkRatioState: [{ name: "test", value: 1000, checked: true }],
            defState: [],
            defRatioState: [],
            hpState: [],
            hpRatioState: [],
            extraDamageState: [],
            criticalState: [],
            criticalDamageState: [],
            meltEnhanceState: [],
            vaporizeEnhanceState: [],
            defMinusState: [],
            resMinusState: [],
            bonusState: [],
        }
    },
    methods: {
        setValue(analysis) {
            let map = {
                "atkState": "atk",
                "atkRatioState": "atk_ratio",
                "defState": "def",
                "defRatioState": "def_ratio",
                "hpState": "hp",
                "hpRatioState": "hp_ratio",
                "extraDamageState": "extra_damage",
                "criticalState": "critical",
                "criticalDamageState": "critical_damage",
                "meltEnhanceState": "melt_enhance",
                "vaporizeEnhanceState": "vaporize_enhance",
                "bonusState": "bonus",
                "defMinusState": "def_minus",
                "resMinusState": "res_minus"
            }
            this.element = analysis.element
            this.damageType = "normal"
            for (let key in map) {
                let fromKey = map[key]
                let temp = []
                for (let i in analysis[fromKey]) {
                    temp.push({
                        name: i,
                        checked: true,
                        value: analysis[fromKey][i]
                    })
                }
                this[key] = temp
            }
        }
    },
    computed: {
        normalDamageName() {
            const map = {
                "Pyro": "火元素伤害",
                "Electro": "雷元素伤害",
                "Hydro": "水元素伤害",
                "Anemo": "风元素伤害",
                "Geo": "岩元素伤害",
                "Dendro": "草元素伤害",
                "Cryo": "冰元素伤害",
                "Physical": "物理伤害"
            }
            return map[this.element]
        },

        showMeltOption() {
            return this.element === "Cryo" || this.element === "Pyro"
        },

        showVaporizeOption() {
            return this.element === "Pyro" || this.element === "Hydro"
        },

        reactionRatio() {
            let map = {
                "Cryomelt": 1.5,
                "Pyromelt": 2,
                "Pyrovaporize": 1.5,
                "Hydrovaporize": 2
            }

            return map[this.element + this.damageType]
        },

        atk() {
            return sum(this.atkState)
        },

        atkRatio() {
            return sum(this.atkRatioState)
        },

        def() {
            return sum(this.defState)
        },

        defRatio() {
            return sum(this.defRatioState)
        },

        hp() {
            return sum(this.hpState)
        },

        hpRatio() {
            return sum(this.hpRatioState)
        },

        extraDamage() {
            return sum(this.extraDamageState)
        },

        bonus() {
            return sum(this.bonusState)
        },

        critical() {
            return sum(this.criticalState)
        },

        criticalDamage() {
            return sum(this.criticalDamageState)
        },

        meltEnhance() {
            return sum(this.meltEnhanceState)
        },

        vaporizeEnhance() {
            return sum(this.vaporizeEnhanceState)
        },

        defMinus() {
            return sum(this.defMinusState)
        },

        resMinus() {
            return sum(this.resMinusState)
        },

        baseDamage() {
            return this.atk * this.atkRatio + this.def * this.defRatio + this.hp * this.hpRatio
        },

        damageNormal() {
            const d = this.baseDamage * (1 + this.critical * this.criticalDamage) * (1 + this.bonus)
            return Math.round(d)
        },

        damageMelt() {
            const d = this.damageNormal * this.reactionRatio * (1 + this.meltEnhance)
            return Math.round(d)
        },

        damageVaporize() {
            const d = this.damageNormal * this.reactionRatio * (1 + this.vaporizeEnhance)
            return Math.round(d)
        }
    }
}
</script>

<style scoped lang="scss">
.header-row {
    display: flex;
    // align-items: center;
}

.big-title {
    height: 32px;
    display: flex;
    justify-content: center;
    align-items: center;
    min-width: 100px;

    &.base-damage-region {
        background-color: rgb(255, 173, 173);
    }

    &.critical-region {
        background-color: rgb(151, 208, 255);
    }

    &.bonus-region {
        background-color: rgb(189, 194, 255);
    }

    &.reaction-ratio-region {
        background-color: #00000022;
    }

    &.vaporize-region {
        background-color: rgb(202, 228, 253);
    }

    &.melt-region {
        background-color: rgb(179, 255, 251);
    }
}
</style>