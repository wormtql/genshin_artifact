import {useArtifactStore} from "@/store/pinia/artifact"
import type {ArtifactSetName, IArtifact, IArtifactWasm} from "@/types/artifact"
// @ts-ignore
import {artifactsData} from "@artifact"
import {convertArtifact, convertArtifactName} from "@/utils/converter"
import {toSnakeCase} from "@/utils/common"
import {newDefaultArtifactConfigForWasm} from "@/utils/artifacts"
import {useI18n} from "@/i18n/i18n"

interface ArtifactGroupItem {
    id: string
    artIds: number[]
    artConfig: any
}

export function use5Artifacts() {
    const { t } = useI18n()
    const artifactStore = useArtifactStore()

    let groupIdCounter = 0

    function getGroupDefault(): ArtifactGroupItem {
        return {
            id: String(groupIdCounter++),
            artIds: [-1, -2, -3, -4, -5],
            artConfig: null,
        }
    }

    const artifactGroups = reactive([
        getGroupDefault(),
        getGroupDefault(),
    ])
    const currentGroupId = ref(artifactGroups[0].id)

    function resetArtifactGroups() {
        artifactGroups.unshift(getGroupDefault(), getGroupDefault())
        currentGroupId.value = artifactGroups[0].id
        artifactGroups.splice(2)
    }

    function loadArtifactGroups(groups: number[][]) {
        artifactGroups.unshift(...groups.map(artIds => ({
            ...getGroupDefault(),
            artIds: artIds.slice(),
        })))
        currentGroupId.value = artifactGroups[0].id
        artifactGroups.splice(groups.length)
    }

    function addArtifactGroup() {
        const group = getGroupDefault()
        artifactGroups.push(group)
        currentGroupId.value = group.id
    }

    function removeArtifactGroup(id: string) {
        for (let i = 0; i < artifactGroups.length; i++) {
            const group = artifactGroups[i]
            if (group.id === id) {
                if (currentGroupId.value === id) {
                    const nextGroup = artifactGroups[i + 1] ?? artifactGroups[i - 1]
                    currentGroupId.value = nextGroup.id
                }
                artifactGroups.splice(i, 1)
                break
            }
        }
    }

    const currentGroup = computed(() => artifactGroups.find(g => g.id === currentGroupId.value)!)

    const artifactIds = computed({
        get: () => currentGroup.value.artIds,
        set: value => currentGroup.value.artIds = value
    })
    // artifact set 4 config
    const artifactSingleConfig = computed({
        get: () => currentGroup.value.artConfig,
        set: value => currentGroup.value.artConfig = value
    })

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

    const artifactCount = computed(() => {
        let count = 0
        for (const id of artifactIds.value) {
            if (id !== -1) {
                count += 1
            }
        }
        return count
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

    watch(() => ({
        groupId: currentGroupId.value,
        newName: artifactNeedConfig4.value,
    }), ({ groupId, newName }) => {
        if (!newName) {
            currentGroup.value.artConfig = null
        } else {
            const data = artifactsData[newName]

            const defaultConfig: any = {}
            for (const c of data.config4) {
                defaultConfig[c.name] = c.default
            }

            const nameWasm = convertArtifactName(newName)
            const configItemName = `config_${toSnakeCase(nameWasm)}`
            if (!currentGroup.value.artConfig || !currentGroup.value.artConfig[configItemName]) {
                currentGroup.value.artConfig = {
                    [configItemName]: defaultConfig
                }
            }
        }
    }, {
        flush: "sync"
    })

    return {
        artifactGroups,
        currentGroupId,
        resetArtifactGroups,
        loadArtifactGroups,
        addArtifactGroup,
        removeArtifactGroup,

        artifactIds,
        artifactCount,
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
