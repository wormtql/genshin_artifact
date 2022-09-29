<template>
    <div class="root">
        <div class="top">
            <select-preset v-model="presetName" style="margin-right: 8px"></select-preset>
            <el-button @click="doCalculation" type="primary">Calc</el-button>
        </div>
        <div class="right">
<!--            <div v-for="item in calcResult">-->
<!--                {{ item }}-->
<!--            </div>-->
            <el-table :data="calcResult">
                <el-table-column label="序号" width="64">
                    <template #default="{ $index }">
                        {{ $index + 1 }}
                    </template>
                </el-table-column>

                <el-table-column label="套装" min-width="120">
                    <template #default="{ row }">
                        <artifact-set-type-display :setType="row.setType"></artifact-set-type-display>
                    </template>
                </el-table-column>

                <el-table-column label="时之沙">
                    <template #default="{ row }">
                        {{ translateStatName(row.stats[0]) }}
                    </template>
                </el-table-column>

                <el-table-column label="空之杯">
                    <template #default="{ row }">
                        {{ translateStatName(row.stats[1]) }}
                    </template>
                </el-table-column>

                <el-table-column label="理之冠">
                    <template #default="{ row }">
                        {{ translateStatName(row.stats[2]) }}
                    </template>
                </el-table-column>

                <el-table-column label="值/百分数">
                    <template #default="{ row }">
                        {{ row.value.toFixed(1) }}/{{ (row.ratio * 100).toFixed(1) }}%
                    </template>
                </el-table-column>
            </el-table>
        </div>
    </div>
</template>

<script lang="ts" setup>
import {useMona} from "@/wasm/mona"
import SelectPreset from "@/components/select/SelectPreset"
import {usePresetStore} from "@/store/pinia/preset"
import {wasmCalcBestArtifactSet} from "@/wasm/calc_best_artifact_set"
import ArtifactSetTypeDisplay from "@/components/display/ArtifactSetTypeDisplay.vue"
import {useI18n} from "@/i18n/i18n";
import {convertArtifactStatNameBack} from "@/utils/converter"
import {ArtifactStatNameWasm} from "@/types/artifact";

// i18n
const { t } = useI18n()

// mona
const mona = await useMona()

// preset
const presetStore = usePresetStore()
const presetName = ref("")

const presetItem = computed(() => {
    if (presetName.value === "") {
        return undefined
    }
    return presetStore.getPreset(presetName.value)
})

const calcBestArtifactSetInterface = computed(() => {
    const item = presetItem.value?.item
    if (!item) {
        return null
    }

    let artifactConfig = null
    if (item.artifactEffectMode === "custom") {
        artifactConfig = item.artifactConfig
    }

    const ret = {
        character: item.character,
        weapon: item.weapon,
        target_function: item.targetFunction,
        buffs: item.buffs,
        enemy: null,
        artifact_config: artifactConfig,
    }
    // console.log(ret)
    return ret
})

// calculation result
interface ResultType {
    stats: string[],
    value: number,
    setType: any,
    ratio: number,
}
const calcResult = ref([] as ResultType[])

async function doCalculation() {
    const loading = ElLoading.service({
        lock: true,
        fullscreen: true,
        text: "莫娜占卜中"
    })

    const result = await wasmCalcBestArtifactSet(calcBestArtifactSetInterface.value, 120000)
    result.reverse()
    const maxValue = result[0].value

    const temp = []
    for (let i = 0; i < result.length; i++) {
        const item = result[i]
        temp.push({
            stats: item.stats,
            value: item.value,
            setType: item.set_type,
            ratio: item.value / maxValue
        })
    }
    calcResult.value = temp

    // console.log(result)

    loading.close();
}

// table
function translateStatName(wasmStatName: ArtifactStatNameWasm): string {
    const monaStatName = convertArtifactStatNameBack(wasmStatName)
    return t("stat", monaStatName)
}

</script>

<style scoped lang="scss">
.top {
    display: flex;
    align-items: center;
    padding-bottom: 16px;
}

.main-stat-span {
    margin-right: 4px;

    &:last-of-type {
        margin-right: 0;
    }
}
</style>