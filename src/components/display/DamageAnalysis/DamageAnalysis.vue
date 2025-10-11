<template>
    <div style="margin-bottom: 16px;" class="flex-row">
        <el-radio-group v-model="damageType" style="margin-right: 24px;">
            <el-radio-button label="normal">{{ normalDamageName }}</el-radio-button>
            <el-radio-button v-if="showMeltOption" label="melt">融化</el-radio-button>
            <el-radio-button v-if="showVaporizeOption" label="vaporize">蒸发</el-radio-button>
            <el-radio-button v-if="showSpreadOption" label="spread">蔓激化</el-radio-button>
            <el-radio-button v-if="showAggravateOption" label="aggravate">超激化</el-radio-button>
            <el-radio-button v-if="showMoonfallOption" label="moonfall">月绽放</el-radio-button>
            <el-radio-button v-if="showMoonElectroOption" label="moonelectro">月感电</el-radio-button>
            <el-radio-button v-if="showDirectMoonElectroOption" label="direct_moonelectro">直伤月感电</el-radio-button>
        </el-radio-group>

        <span class="damage-display" v-if="damageType === 'normal'">{{ Math.round(damageNormal) }}</span>
        <span class="damage-display" v-if="damageType === 'melt'">{{ Math.round(damageMelt) }}</span>
        <span class="damage-display" v-if="damageType === 'vaporize'">{{ Math.round(damageVaporize) }}</span>
        <span class="damage-display" v-if="damageType === 'spread'">{{ Math.round(damageSpread) }}</span>
        <span class="damage-display" v-if="damageType === 'aggravate'">{{ Math.round(damageAggravate) }}</span>
        <span class="damage-display" v-if="damageType === 'moonfall'">{{ Math.round(damageMoonfall) }}</span>
        <span class="damage-display" v-if="damageType === 'moonelectro'">{{ Math.round(damageMoonElectro) }}</span>
        <span class="damage-display" v-if="damageType === 'direct_moonelectro'">{{ Math.round(damageDirectMoonElectro) }}</span>
    </div>

    <div class="header-row" style="overflow: auto; margin-bottom: 16px;">
        <div>
            <div class="big-title base-damage-region" :title="Math.round(baseDamageSpread*1000)/1000" v-if="damageType === 'spread'">{{ baseRegionName }}</div>
            <div class="big-title base-damage-region" :title="Math.round(baseDamageAggravate*1000)/1000" v-else-if="damageType === 'aggravate'">{{ baseRegionName }}</div>
            <div class="big-title base-damage-region" :title="Math.round(baseDamageMoonfall*1000)/1000" v-else-if="damageType === 'moonfall'">{{ baseRegionName }}</div>
            <div class="big-title base-damage-region" :title="Math.round(baseDamageMoonElectro*1000)/1000" v-else-if="damageType === 'moonelectro'">{{ baseRegionName }}</div>
            <div class="big-title base-damage-region" :title="Math.round(baseDamageDirectMoonElectro*1000)/1000" v-else-if="damageType === 'direct_moonelectro'">{{ baseRegionName }}</div>
            <div class="big-title base-damage-region" :title="Math.round(baseDamage*1000)/1000" v-else>{{ baseRegionName }}</div>
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
                    v-if="emRatioState.length > 0"
                    :arr="emState"
                    title="元素精通"
                ></damage-analysis-util>
                <damage-analysis-util
                    v-if="emRatioState.length > 0"
                    :arr="emRatioState"
                    title="元素精通倍率"
                ></damage-analysis-util>
                <damage-analysis-util
                    v-if="extraDamageState.length > 0"
                    :arr="extraDamageState"
                    title="其他"
                ></damage-analysis-util>
                <div v-if="damageType === 'spread'" style="min-width: 100px">
                    <div class="big-title" style="background: rgb(236, 245, 255)">蔓激化基础伤害</div>
                    <div class="header-row" style="height: 100%; display: flex; align-items: center; justify-content: center">
                        <span>{{ Math.round(baseDamageQuicken * 1000) / 1000 }}</span>
                    </div>
                </div>
                <div v-if="damageType === 'aggravate'" style="min-width: 100px">
                    <div class="big-title" style="background: rgb(236, 245, 255)">超激化基础伤害</div>
                    <div class="header-row" style="height: 100%; display: flex; align-items: center; justify-content: center">
                        <span>{{ Math.round(baseDamageQuicken * 1000) / 1000 }}</span>
                    </div>
                </div>
                <div v-if="damageType === 'moonfall'" style="min-width: 100px">
                    <div class="big-title" style="background: rgb(236, 245, 255)">月绽放基础伤害</div>
                    <div class="header-row" style="height: 100%; display: flex; align-items: center; justify-content: center">
                        <span>{{ Math.round(baseDamageMoonQuicken * 1000) / 1000 }}</span>
                    </div>
                </div>
                <div v-if="damageType === 'moonelectro'" style="min-width: 100px">
                    <div class="big-title" style="background: rgb(236, 245, 255)">月感电基础伤害</div>
                    <div class="header-row" style="height: 100%; display: flex; align-items: center; justify-content: center">
                        <span>{{ Math.round(baseDamageMoonQuicken * 1000) / 1000 }}</span>
                    </div>
                </div>
                <div v-if="damageType === 'direct_moonelectro'" style="min-width: 100px">
                    <div class="big-title" style="background: rgb(236, 245, 255)">直伤月感电基础</div>
                    <div class="header-row" style="height: 100%; display: flex; align-items: center; justify-content: center">
                        <span>{{ Math.round(3.0 * 1000) / 1000 }}倍</span>
                    </div>
                </div>
                <damage-analysis-util
                    v-if="damageType === 'spread'"
                    :arr="spreadState"
                    title="蔓激化伤害提升"
                ></damage-analysis-util>
                <damage-analysis-util
                    v-if="damageType === 'aggravate'"
                    :arr="aggravateState"
                    title="超激化伤害提升"
                ></damage-analysis-util>
                <damage-analysis-util
                    v-if="damageType === 'moonfall'"
                    :arr="moonfallState"
                    title="月绽放伤害提升"
                ></damage-analysis-util>
                <damage-analysis-util
                    v-if="damageType === 'moonfall'"
                    :arr="enhanceMoonReactionState"
                    title="月曜反应增伤"
                ></damage-analysis-util>
                <damage-analysis-util
                    v-if="damageType === 'moonelectro'"
                    :arr="moonElectroState"
                    title="月感电伤害提升"
                ></damage-analysis-util>
                <damage-analysis-util
                    v-if="damageType === 'moonelectro'"
                    :arr="enhanceMoonReactionState"
                    title="月曜反应增伤"
                ></damage-analysis-util>
                <damage-analysis-util
                    v-if="damageType === 'moonelectro'"
                    :arr="moonElectroBaseState"
                    title="月感电基础伤害提升"
                ></damage-analysis-util>
                <damage-analysis-util
                    v-if="damageType === 'direct_moonelectro'"
                    :arr="directMoonElectroState"
                    title="直伤月感电增伤"
                ></damage-analysis-util>
                <damage-analysis-util
                    v-if="damageType === 'direct_moonelectro'"
                    :arr="enhanceMoonReactionState"
                    title="月曜反应增伤"
                ></damage-analysis-util>
                <damage-analysis-util
                    v-if="damageType === 'direct_moonelectro'"
                    :arr="moonElectroBaseState"
                    title="月感电基础伤害提升"
                ></damage-analysis-util>
            </div>
        </div>
        <div v-show="isDamage">
            <div class="big-title critical-region" :title="Math.round(this.critical * this.criticalDamage * 1000)/1000">暴击</div>
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
        <div v-if="damageType !== 'moonelectro' && damageType !== 'direct_moonelectro' && damageType !== 'moonfall'">
            <div class="big-title bonus-region">加成</div>
            <div class="header-row">
                <damage-analysis-util
                    :arr="bonusRegionState"
                    :title="bonusRegionName"
                ></damage-analysis-util>
            </div>
        </div>
        <div v-if="damageType === 'melt' || damageType === 'vaporize'">
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
                    :arr="vaporizeEnhanceState"
                    title="蒸发伤害加成"
                ></damage-analysis-util>
            </div>
        </div>
    </div>

    <div v-if="isDamage" class="header-row" style="overflow: auto">
        <div>
            <div class="big-title def-minus">防御乘区</div>
            <div class="header-row">
                <damage-analysis-util
                    :arr="defMinusState"
                    title="减防"
                ></damage-analysis-util>
                <damage-analysis-util
                    :arr="defPenetrationState"
                    title="穿防"
                ></damage-analysis-util>
            </div>
        </div>

        <div>
            <div class="big-title res-minus">抗性乘区</div>
            <div class="header-row">
                <damage-analysis-util
                    :arr="resMinusState"
                    title="减抗"
                ></damage-analysis-util>
            </div>
        </div>
    </div>
</template>

<script>
import DamageAnalysisUtil from "./DamageAnalysisUtil"
import { LEVEL_MULTIPLIER } from "@/constants/levelMultiplier"

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
    props: ["enemyConfig", "characterLevel"],
    data() {
        return {
            damageType: "normal",
            element: "Pyro",
            isHeal: false,
            isShield: false,
            isDamage: true,

            atkState: [{ name: "test", value: 1000, checked: true }],
            atkRatioState: [{ name: "test", value: 1000, checked: true }],
            defState: [],
            defRatioState: [],
            hpState: [],
            hpRatioState: [],
            emState: [],
            emRatioState: [],
            extraDamageState: [],
            spreadState: [],
            aggravateState: [],
            moonfallState: [],
            moonElectroState: [],
            moonElectroBaseState: [], // 月感电基础伤害提升状态
            directMoonElectroState: [], // 直伤月感电增伤状态
            enhanceMoonReactionState: [], // 月曜反应通用增伤状态
            directMoonElectroRatioState: [], // 直伤月感电倍率状态
            criticalState: [],
            criticalDamageState: [],
            meltEnhanceState: [],
            vaporizeEnhanceState: [],
            defMinusState: [],
            defPenetrationState: [],
            resMinusState: [],
            bonusState: [],
            healingBonusState: []
        }
    },
    methods: {
        setValue(analysis) {
            console.log(analysis)
            let map = {
                "atkState": "atk",
                "atkRatioState": "atk_ratio",
                "defState": "def",
                "defRatioState": "def_ratio",
                "hpState": "hp",
                "hpRatioState": "hp_ratio",
                "emState": "em",
                "emRatioState": "em_ratio",
                "extraDamageState": "extra_damage",
                "criticalState": "critical",
                "criticalDamageState": "critical_damage",
                "meltEnhanceState": "melt_enhance",
                "vaporizeEnhanceState": "vaporize_enhance",
                "bonusState": "bonus",
                "defMinusState": "def_minus",
                "defPenetrationState": "def_penetration",
                "resMinusState": "res_minus",
                "healingBonusState": "healing_bonus",
                "aggravateState": "aggravate_compose",
                "spreadState": "spread_compose",
                "moonfallState": "moonfall_compose",
                "moonElectroState": "moonelectro_compose",
                "moonElectroBaseState": "moonelectro_base_compose", // 月感电基础伤害提升映射
                "directMoonElectroState": "direct_moonelectro_compose", // 直伤月感电增伤映射
                "enhanceMoonReactionState": "enhance_moon_reaction_compose", // 月曜反应通用增伤映射
                "directMoonElectroRatioState": "direct_moonelectro_ratio", // 直伤月感电倍率映射
            }
            this.element = analysis.element
            this.isHeal = analysis.is_heal
            this.isShield = analysis.is_shield
            this.isDamage = !this.isHeal && !this.isShield
            this.damageType = "normal"
            for (let key in map) {
                let fromKey = map[key]
                let temp = []
                for (let i in analysis[fromKey]) {
                    temp.push({
                        name: i,
                        checked: true,
                        value: Math.round(analysis[fromKey][i] * 1000) / 1000
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
            if (this.isHeal) {
                return "治疗量"
            } else if (this.isShield) {
                return "护盾量"
            } else {
                return map[this.element]
            }
        },

        showMeltOption() {
            return (this.element === "Cryo" || this.element === "Pyro") && this.isDamage
        },

        showVaporizeOption() {
            return (this.element === "Pyro" || this.element === "Hydro") && this.isDamage
        },

        showSpreadOption() {
            return this.element === "Dendro"
        },

        showAggravateOption() {
            return this.element === "Electro"
        },

        showMoonfallOption() {
            return this.element === "Dendro" || this.element === "Hydro"
        },

        showMoonElectroOption() {
            console.log("Current element:", this.element, "Show moonelectro:", this.element === "Electro" || this.element === "Hydro")
            return this.element === "Electro" || this.element === "Hydro"
        },

        showDirectMoonElectroOption() {
            return this.element === "Electro" || this.element === "Hydro"
        },
        
        baseRegionName() {
            if (this.isHeal) {
                return "基础治疗"
            } else {
                return "基础伤害"
            }
        },

        bonusRegionState() {
            if (this.isHeal) {
                return this.healingBonusState
            } else {
                return this.bonusState
            }
        },

        bonusRegionName() {
            if (this.isHeal) {
                return "治疗加成"
            } else {
                return "伤害加成"
            }
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

        em() {
            return sum(this.emState)
        },

        emRatio() {
            return sum(this.emRatioState)
        },

        extraDamage() {
            return sum(this.extraDamageState)
        },

        bonus() {
            return sum(this.bonusState)
        },

        healingBonus() {
            return sum(this.healingBonusState)
        },

        critical() {
            return Math.min(sum(this.criticalState), 1)
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

        defPenetration() {
            return sum(this.defPenetrationState)
        },

        resMinus() {
            return sum(this.resMinusState)
        },

        baseDamage() {
            return this.atk * this.atkRatio + this.def * this.defRatio + this.hp * this.hpRatio + this.em * this.emRatio + this.extraDamage;
        },

        spreadEnhance() {
            return sum(this.spreadState)
        },

        aggravateEnhance() {
            console.log(this.aggravateState)
            return sum(this.aggravateState)
        },

        moonfallEnhance() {
            return sum(this.moonfallState)
        },

        moonElectroEnhance() {
            return sum(this.moonElectroState)
        },

        moonElectroBaseEnhance() {
            return sum(this.moonElectroBaseState)
        },

        enhanceMoonReaction() {
            return sum(this.enhanceMoonReactionState)
        },

        levelMultiplier() {
            // 与后端 LEVEL_MULTIPLIER 对应
            return LEVEL_MULTIPLIER[this.characterLevel - 1]
        },

        baseDamageSpread() {
            return this.baseDamage + LEVEL_MULTIPLIER[this.characterLevel - 1] * 1.25 * (1 + this.spreadEnhance)
        },

        baseDamageAggravate() {
            return this.baseDamage + LEVEL_MULTIPLIER[this.characterLevel - 1] * 1.15 * (1 + this.aggravateEnhance)
        },

        baseDamageMoonfall() {
            // 月绽放反应的基础伤害计算
            // 使用反应专用的计算，不是基于技能倍率
            return this.baseDamageMoonQuicken
        },

        baseDamageMoonElectro() {
            // 月感电反应的基础伤害计算  
            // 使用反应专用的计算，不是基于技能倍率
            return this.baseDamageMoonQuicken
        },

        baseDamageDirectMoonElectro() {
            // 直伤月感电的基础伤害显示（仅用于UI显示，实际计算在damageDirectMoonElectro中）
            const totalRatio = sum(this.directMoonElectroRatioState) // 使用专门的直伤月感电倍率
            const enhancedRatio = totalRatio * (1.0 + this.moonElectroBaseEnhance)
            return 3.0 * this.atk * enhancedRatio
        },

        baseDamageQuicken() {
            return LEVEL_MULTIPLIER[this.characterLevel - 1] * (this.damageType === "spread" ? 1.25 : 1.15)
        },

        baseDamageMoonQuicken() {
            // 月感电和月绽放的基础伤害计算
            // 基于角色等级的反应伤害倍率
            const levelMultipliers = {
                90: 1446.85, 80: 1077.44, 70: 765.64, 60: 550.52, 50: 401.54,
                40: 293.55, 30: 215.05, 20: 157.67, 10: 115.48, 1: 17.17
            }
            
            const baseDamage = levelMultipliers[this.characterLevel] || levelMultipliers[90]
            
            if (this.damageType === "moonfall") {
                // 月绽放：基础倍率 2.0
                return baseDamage * 2.0
            } else if (this.damageType === "moonelectro") {
                // 月感电：基础倍率 1.8 (与后端一致)
                return baseDamage * 1.8
            } else if (this.damageType === "direct_moonelectro") {
                // 直伤月感电：基础倍率 1.2 (仅用于计算基础，实际不基于等级)
                return baseDamage * 1.2
            }
            
            return 0
        },

        // 元素精通对反应伤害的加成
        reactionEMBonus() {
            // 元素精通加成公式: 2.78 * EM / (EM + 1400)
            return 2.78 * this.em / (this.em + 1400)
        },

        resRatio() {
            // default res to 0.1
            // console.log(this.enemyConfig)
            const originalRes = this.enemyConfig[this.element.toLowerCase() + "_res"]
            const res = originalRes - this.resMinus
            let res_ratio
            if (res > 0.75) {
                res_ratio = 1 / (1 + res * 4)
            } else if (res > 0) {
                res_ratio = 1 - res
            } else {
                res_ratio = 1 - res / 2
            }
            return res_ratio
        },

        defMultiplier() {
            const enemyLevel = this.enemyConfig.level
            const characterLevel = this.characterLevel
            const c = 100 + characterLevel
            return c / ((1 - this.defPenetration) * (1 - this.defMinus) * (100 + enemyLevel) + c)
        },

        damageSpread() {
            return this.baseDamageSpread * (1 + this.critical * this.criticalDamage) * (1 + this.bonus) * this.resRatio * this.defMultiplier
        },

        damageAggravate() {
            return this.baseDamageAggravate * (1 + this.critical * this.criticalDamage) * (1 + this.bonus) * this.resRatio * this.defMultiplier
        },

        damageMoonfall() {
            // 月绽放反应伤害计算
            // 反应伤害 = 基础反应伤害 * (1 + 元素精通加成 + 反应伤害加成 + 月曜反应通用增伤) * 抗性乘区
            // 注意：反应伤害不受暴击、攻击力、伤害加成影响
            const reactionDamage = this.baseDamageMoonQuicken * (1 + this.reactionEMBonus + this.moonfallEnhance + this.enhanceMoonReaction)
            return reactionDamage * this.resRatio
        },

        damageMoonElectro() {
            // 月感电反应伤害计算（与后端一致）
            // 基础伤害 = 等级倍率 * 基础倍率(1.8) * (1 + 基础伤害提升) * (1 + 月感电增伤 + 月曜反应通用增伤)
            // 可以暴击，但不受常规伤害加成影响
            const baseMultiplier = 1.8
            const enhancedBaseMultiplier = baseMultiplier * (1.0 + this.moonElectroBaseEnhance)
            const moonelectroBaseDamage = this.levelMultiplier * enhancedBaseMultiplier * (1.0 + this.moonElectroEnhance + this.enhanceMoonReaction)
            
            // 应用暴击（期望伤害）
            const finalDamage = moonelectroBaseDamage * (1.0 + this.critical * this.criticalDamage)
            return finalDamage * this.resRatio
        },

        damageDirectMoonElectro() {
            // 直伤月感电计算（与后端一致）
            // 基础伤害 = 3 × 攻击力 × 倍率 × (1+基础提升%) × (1+直伤月感电增伤% + 月曜反应通用增伤%)
            // 注意：基于攻击力，不是等级倍率；可以暴击，不受常规伤害加成影响
            const directMoonElectroEnhance = sum(this.directMoonElectroState)
            const totalRatio = sum(this.directMoonElectroRatioState) // 使用专门的直伤月感电倍率
            const multiplier3x = 3.0
            const enhancedRatio = totalRatio * (1.0 + this.moonElectroBaseEnhance)
            const directMoonelectroBaseDamage = multiplier3x * this.atk * enhancedRatio * (1.0 + directMoonElectroEnhance + this.enhanceMoonReaction)
            
            // 应用暴击（期望伤害）
            const finalDamage = directMoonelectroBaseDamage * (1.0 + this.critical * this.criticalDamage)
            return finalDamage * this.resRatio
        },

        damageNormal() {
            let d
            if (this.isHeal) {
                d = this.baseDamage * (1 + this.healingBonus)
            } else {
                d = this.baseDamage * (1 + this.critical * this.criticalDamage) * (1 + this.bonus) * this.resRatio * this.defMultiplier
            }
            return d
        },

        damageMelt() {
            const d = this.damageNormal * this.reactionRatio * (1 + this.meltEnhance)
            return d
        },

        damageVaporize() {
            const d = this.damageNormal * this.reactionRatio * (1 + this.vaporizeEnhance)
            return d
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
        background-color: rgb(217, 236, 255);
    }

    &.critical-region {
        background-color: rgb(179, 216, 255);
    }

    &.bonus-region {
        background-color: rgb(217, 236, 255);
    }

    &.reaction-ratio-region {
        background-color: rgb(179, 216, 255);
    }

    &.vaporize-region {
        background-color: rgb(217, 236, 255);
    }

    &.melt-region {
        background-color: rgb(217, 236, 255);
    }

    &.def-minus {
        background-color: rgb(217, 236, 255);
    }

    &.res-minus {
        background-color: rgb(179, 216, 255);
    }
}
</style>