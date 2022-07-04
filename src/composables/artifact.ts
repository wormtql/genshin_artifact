import {useArtifactStore} from "@/store/pinia/artifact"
import type {ArtifactSetName, IArtifact, IArtifactWasm} from "@/types/artifact"
// @ts-ignore
import {artifactsData} from "@artifact"
import {convertArtifact, convertArtifactName} from "@/utils/converter"
import {toSnakeCase} from "@/utils/common"
import {newDefaultArtifactConfigForWasm} from "@/utils/artifacts"
import {useI18n} from "@/i18n/i18n"

export function use5Artifacts() {
    const { t } = useI18n()
    const artifactStore = useArtifactStore()

    const artifactIds = ref([-1, -1, -1, -1, -1])
    // artifact set 4 config
    const artifactSingleConfig = ref<any>(null)

    const artifactItems = computed(() => {
        let temp: (IArtifact | null)[] = []
        for (let id of artifactIds.value) {
            if (id >= 0) {
                const a = artifactStore.artifacts.value.get(id)
                if (a) {
                    temp.push(a)
                } else {
                    temp.push(null)
                }
            } else {
                temp.push(null)
            }
        }
        return temp
    })

    const artifactSetCount = computed(() => {
        let temp: Record<ArtifactSetName, number> = {}
        for (let artifact of artifactItems.value) {
            if (!artifact) {
                continue
            }
            const setName = artifact.setName
            if (!Object.prototype.hasOwnProperty.call(temp, setName)) {
                temp[setName] = 0
            }
            temp[setName] += 1
        }
        return temp
    })

    const artifactNeedConfig4 = computed((): ArtifactSetName | null => {
        for (let setName in artifactSetCount.value) {
            const count = artifactSetCount.value[setName]
            if (count >= 4) {
                const data = artifactsData[setName]
                if (data.config4 && data.config4.length > 0) {
                    return setName
                }
            }
        }

        return null
    })

    const artifactConfig4ItemName = computed((): string | null => {
        if (artifactNeedConfig4.value) {
            const setNameWasm = convertArtifactName(artifactNeedConfig4.value)
            return `config_${toSnakeCase(setNameWasm)}`
        }
        return null
    })

    const artifactEffect4Text = computed((): string => {
        if (!artifactNeedConfig4.value) {
            return ""
        }
        // const data = artifactsData[artifactNeedConfig4.value]
        // return data.effect4
        return t("artifact", artifactNeedConfig4.value, "effects", 4)
    })

    const artifactConfig4Configs = computed(() => {
        if (artifactNeedConfig4.value) {
            const data = artifactsData[artifactNeedConfig4.value]
            return data.config4
        }
        return []
    })

    const artifactWasmFormat = computed((): IArtifactWasm[] => {
        let temp: IArtifactWasm[] = []
        for (let id of artifactIds.value) {
            if (id >= 0) {
                const a = artifactStore.artifacts.value.get(id)
                if (a && !a.omit) {
                    const artifactWasm = convertArtifact(a)
                    temp.push(artifactWasm)
                }
            }
        }
        return temp
    })

    const artifactConfigForCalculator = computed(() => {
        let base = newDefaultArtifactConfigForWasm()

        if (artifactNeedConfig4.value) {
            let name = artifactConfig4ItemName.value as string
            base[name] = artifactSingleConfig.value[name]
        }

        return base
    })

    function setArtifact(index: number, id: number) {
        artifactIds.value[index] = id
    }

    function removeArtifact(index: number) {
        artifactIds.value[index] = -1
    }

    function toggleArtifact(index: number) {
        const a = artifactItems.value[index]
        if (a) {
            const id = a.id
            artifactStore.toggleArtifact(id)
        }
    }

    watch(() => artifactNeedConfig4.value, newName => {
        if (!newName) {
            artifactSingleConfig.value = null
        } else {
            const data = artifactsData[newName]

            let defaultConfig: any = {}
            for (let c of data.config4) {
                defaultConfig[c.name] = c.default
            }

            const nameWasm = convertArtifactName(newName)
            const configItemName = `config_${toSnakeCase(nameWasm)}`
            artifactSingleConfig.value = {
                [configItemName]: defaultConfig
            }
        }
    }, {
        flush: "sync"
    })

    return {
        artifactIds,
        artifactSingleConfig,
        artifactWasmFormat,

        artifactItems,
        artifactSetCount,
        artifactNeedConfig4,
        artifactConfig4ItemName,
        artifactEffect4Text,
        artifactConfig4Configs,
        artifactConfigForCalculator,

        setArtifact,
        removeArtifact,
        toggleArtifact,
    }
}