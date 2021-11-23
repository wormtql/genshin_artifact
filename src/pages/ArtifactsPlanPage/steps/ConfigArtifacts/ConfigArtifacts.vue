<template>
    <div>
        <el-alert
            title="注：除非你知道你在做什么，否则不建议修改此处参数"
            type="warning"
            :closable="false"
            class="artifact-config-item"
        ></el-alert>

        <common-artifact-config-ratio
            artifact-name="huskOfOpulentDreams"
            v-model="configHuskOfOpulentDreams.level"
            :max="4"
            class="artifact-config-item config-item"
        ></common-artifact-config-ratio>

        <common-artifact-config-ratio
            artifact-name="shimenawaReminiscence"
            v-model="configShimenawaReminiscence.rate"
            class="artifact-config-item config-item"
        ></common-artifact-config-ratio>

        <common-artifact-config-ratio
            artifact-name="braveHeart"
            v-model="configBraveHeart.rate"
            class="artifact-config-item config-item"
        ></common-artifact-config-ratio>

        <common-artifact-config-ratio
            artifact-name="instructor"
            v-model="configInstructor.rate"
            class="artifact-config-item config-item"
        ></common-artifact-config-ratio>

        <common-artifact-config-ratio
            artifact-name="berserker"
            v-model="configBerserker.rate"
            class="artifact-config-item config-item"
        ></common-artifact-config-ratio>

        <common-artifact-config-ratio
            artifact-name="martialArtist"
            v-model="configMartialArtist.rate"
            class="artifact-config-item config-item"
        ></common-artifact-config-ratio>

        <common-artifact-config-ele-multi
            artifact-name="defenderWill"
            v-model="configDefenderWill.elements"
            class="artifact-config-item config-item"
        ></common-artifact-config-ele-multi>

        <common-artifact-config-ratio
            artifact-name="crimsonWitch"
            v-model="configCrimsonWitch.level"
            :max="3"
            class="artifact-config-item config-item"
        ></common-artifact-config-ratio>

        <common-artifact-config-ratio
            artifact-name="blizzardStrayer"
            v-model="configBlizzardStrayer.criticalBonus"
            title="等效暴击率提升"
            class="artifact-config-item config-item"
        ></common-artifact-config-ratio>

        <common-artifact-config-ratio
            artifact-name="bloodstainedChivalry"
            v-model="configBloodstainedChivalry.rate"
            class="artifact-config-item config-item"
        ></common-artifact-config-ratio>

        <common-artifact-config-ratio
            artifact-name="noblesseOblige"
            v-model="configNoblesseOblige.rate"
            class="artifact-config-item config-item"
        ></common-artifact-config-ratio>

        <common-artifact-config-ratio
            artifact-name="heartOfDepth"
            v-model="configHeartOfDepth.rate"
            class="artifact-config-item config-item"
        ></common-artifact-config-ratio>

        <common-artifact-config-ele-ratio
            artifact-name="archaicPetra"
            v-model="configArchaicPetra"
            class="artifact-config-item config-item"
            :elements="['fire', 'thunder', 'water', 'ice']"
        ></common-artifact-config-ele-ratio>

        <common-artifact-config-ratio2
            artifact-name="paleFlame"
            v-model="configPaleFlame"
            title1="4件套被动层数"
            :max1="2"
            attrName1="level"
            title2="叠满2层效果比例"
            attrName2="rate"
            class="artifact-config-item config-item"
        ></common-artifact-config-ratio2>

        <common-artifact-config-ratio
            artifact-name="retracingBolide"
            v-model="configRetracingBolide.rate"
            class="artifact-config-item config-item"
        ></common-artifact-config-ratio>

        <common-artifact-config-ratio
            artifact-name="thunderSmoother"
            v-model="configThunderSmoother.rate"
            class="artifact-config-item config-item"
        ></common-artifact-config-ratio>

        <common-artifact-config-ratio
            artifact-name="tenacityOfTheMillelith"
            v-model="configTenacityOfTheMillelith.rate"
            class="artifact-config-item config-item"
        ></common-artifact-config-ratio>

        <common-artifact-config-ratio
            artifact-name="lavaWalker"
            v-model="configLavaWalker.rate"
            class="artifact-config-item config-item"
        ></common-artifact-config-ratio>

        <common-artifact-config-ratio
            artifact-name="maidenBeloved"
            v-model="configMaidenBeloved.rate"
            class="artifact-config-item config-item"
        ></common-artifact-config-ratio>
    </div>
</template>

<script>
import CommonArtifactConfigRatio from "./configs/CommonArtifactConfigRatio";
import CommonArtifactConfigRatio2 from "./configs/CommonArtifactConfigRatio2";
import CommonArtifactConfigEleRatio from "./configs/CommonArtifactConfigEleRatio";
import CommonArtifactConfigEleMulti from "./configs/CommonArtifactConfigEleMulti";
// import CommonArtifactConfigNumber from "./configs/CommonArtifactConfigNumber";

import deepCopy from "@util/deepcopy";

export default {
    name: "ConfigArtifacts",
    components: {
        CommonArtifactConfigRatio,
        CommonArtifactConfigRatio2,
        CommonArtifactConfigEleRatio,
        CommonArtifactConfigEleMulti,
        // CommonArtifactConfigNumber,
    },
    data() {
        return {
            configShimenawaReminiscence: { rate: 0 },
            configArchaicPetra: { element: "fire", rate: 0 },
            configBraveHeart: { rate: 0 },
            configInstructor: { rate: 0 },
            configBerserker: { rate: 0 },
            configMartialArtist: { rate: 0 },
            configDefenderWill: { elements: [] },
            configCrimsonWitch: { level: 0 },
            configBlizzardStrayer: { criticalBonus: 0 },
            configBloodstainedChivalry: { rate: 0 },
            configNoblesseOblige: { rate: 0 },
            configHeartOfDepth: { rate: 0 },
            configPaleFlame: { level: 0, rate: 0 },
            configRetracingBolide: { rate: 0 },
            configThunderSmoother: { rate: 0 },
            configTenacityOfTheMillelith: { rate: 0 },
            configLavaWalker: { rate: 0 },
            configMaidenBeloved: { rate: 0 },
            configHuskOfOpulentDreams: { level: 0 },
        }
    },
    methods: {
        getArtifactsConfig() {
            return deepCopy(this.$data);
        },

        setArtifactsConfig(config) {
            this.configShimenawaReminiscence = config.configShimenawaReminiscence ?? { rate: 0 };
            this.configArchaicPetra = config.configArchaicPetra ?? { element: "fire", rate: 0 };
            this.configBraveHeart = config.configBraveHeart ?? { rate: 0 };
            this.configInstructor = config.configInstructor ?? { rate: 0 };
            this.configBerserker = config.configBerserker ?? { rate: 0 };
            this.configMartialArtist = config.configMartialArtist ?? { rate: 0 };
            this.configDefenderWill = config.configDefenderWill ?? { elements: [] };
            this.configCrimsonWitch = config.configCrimsonWitch ?? { level: 0 };
            this.configBlizzardStrayer = config.configBlizzardStrayer ?? { criticalBonus: 0 };
            this.configBloodstainedChivalry = config.configBloodstainedChivalry ?? { rate: 0 };
            this.configNoblesseOblige = config.configNoblesseOblige ?? { rate: 0 };
            this.configHeartOfDepth = config.configHeartOfDepth ?? { rate: 0 };
            this.configPaleFlame = config.configPaleFlame ?? { rate: 0 };
            this.configRetracingBolide = config.configRetracingBolide ?? { rate: 0 };
            this.configThunderSmoother = config.configThunderSmoother ?? { rate: 0 };
            this.configTenacityOfTheMillelith = config.configTenacityOfTheMillelith ?? { rate: 0 };
            this.configLavaWalker = config.configLavaWalker ?? { rate: 0 };
            this.configMaidenBeloved = config.configMaidenBeloved ?? { rate: 0 };
        }
    }
}
</script>

<style lang="scss" scoped>
.artifact-config-item {
    margin-bottom: 16px;

    &:last-of-type {
        margin-bottom: 0;
    }
}
</style>