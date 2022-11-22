import {RandomIDProvider} from "@/utils/idProvider"
import { KumiItem } from "@/types/kumi"
import {useArtifactStore} from "@/store/pinia/artifact"
import {positionToIndex} from "@/utils/artifacts"

import { type Ref } from "vue"


const artifactStore = useArtifactStore()

function loadKumiOrDefault(payload: any): KumiItem[] {
    let kumi = {
        0: {
            id: 0,
            title: "默认收藏夹",
            dir: true,
            children: []
        }
    }
    if (payload) {
        try {
            if (typeof payload === "object") {
                if (payload.kumi) {
                    kumi = payload.kumi
                } else {
                    kumi = payload
                }
            } else {
                return payload
            }
        } catch (e) {
            console.error(e)
        }
    }

    let result: KumiItem[] = [...Object.values(kumi)]
    result.sort((a, b) => a.id - b.id)

    return result
}

function store() {
    const kumi = ref(loadKumiOrDefault(null))
    const kumiById: Ref<Map<number, KumiItem>> = ref(new Map())

    const idGenerator = new RandomIDProvider()

    function init(payload: any) {
        kumi.value = loadKumiOrDefault(payload)
        kumiById.value.clear()
        for (let item of kumi.value) {
            kumiById.value.set(item.id, item)
        }
    }

    const dirs = computed((): KumiItem[] => {
        const result = []
        for (let item of kumi.value) {
            if (item.dir) {
                result.push(item)
            }
        }
        return result
    })

    function createDir(name: string) {
        let item: KumiItem = {
            id: idGenerator.generateId(),
            title: name,
            dir: true,
            children: [],
        }

        kumi.value.push(item)
        kumiById.value.set(item.id, item)
    }

    function deleteDir(id: number) {
        const item = kumiById.value.get(id)

        if (item && item.dir) {
            let children = item.children ?? []
            let deleteSet = new Set(children)
            deleteSet.add(id)

            // delete children and self
            let i = 0
            while (i < kumi.value.length) {
                const temp: KumiItem = kumi.value[i]
                if (deleteSet.has(temp.id)) {
                    kumi.value.splice(i, 1)
                    kumiById.value.delete(temp.id)
                } else {
                    i += 1
                }
            }
        }
    }

    function rename(id: number, name: string) {
        const item = kumiById.value.get(id)
        if (item) {
            item.title = name
        }
    }


    const kumisByDirId = computed((): Record<number, KumiItem[]> => {
        let result: Record<number, KumiItem[]> = {}

        for (let item of kumi.value) {
            if (item.dir) {
                result[item.id] = result[item.id] ?? []

                if (item.children) {
                    for (let childId of item.children) {
                        result[item.id].push(kumiById.value.get(childId) as KumiItem)
                    }
                }
            }
        }

        return result
    })

    function addKumi(dirId: number, name: string, artifactIds: number[]): number | null {
        let dir = kumiById.value.get(dirId)
        if (dir) {
            let item: KumiItem = {
                id: idGenerator.generateId(),
                title: name,
                dir: false,
                artifactIds: artifactIds as any,
            }

            kumi.value.push(item)
            kumiById.value.set(item.id, item)

            if (dir.children) {
                dir.children.push(item.id)
            }

            return item.id
        } else {
            return null
        }
    }

    function createKumi(dirId: number, name: string): number | null {
        let dir = kumiById.value.get(dirId)
        if (dir) {
            let item: KumiItem = {
                id: idGenerator.generateId(),
                title: name,
                dir: false,
                artifactIds: [null, null, null, null, null]
            }

            kumi.value.push(item)
            kumiById.value.set(item.id, item)

            if (dir.children) {
                dir.children.push(item.id)
            }

            return item.id
        } else {
            return null
        }
    }

    function deleteKumi(id: number) {
        // delete kumi
        kumiById.value.delete(id)
        const index = kumi.value.findIndex(x => x.id === id)
        if (index > 0) {
            kumi.value.splice(index, 1)
        }

        // remove from parent's children
        for (const item of kumi.value) {
            if (item.dir && item.children) {
                const index = item.children.indexOf(id)
                if (index !== -1) {
                    item.children.splice(index, 1)
                    break
                }
            }
        }
    }

    function addArtifact(kumiId: number, artifactId: number) {
        let item = kumiById.value.get(kumiId)
        if (item) {
            item.artifactIds = item.artifactIds ?? [null, null, null, null, null]
            let artifact = artifactStore.artifacts.value.get(artifactId)
            if (artifact) {
                const position = artifact.position
                const index = positionToIndex(position)
                item.artifactIds[index] = artifactId
            }
        }
    }

    function deleteArtifact(kumiId: number, artifactId: number) {
        let item = kumiById.value.get(kumiId)
        if (item) {
            item.artifactIds = item.artifactIds ?? [null, null, null, null, null]

            let artifact = artifactStore.artifacts.value.get(artifactId)
            if (artifact) {
                const position = artifact.position
                const index = positionToIndex(position)
                item.artifactIds[index] = null
            }
        }
    }

    function itemById(id: number): KumiItem | undefined {
        return kumiById.value.get(id)
    }

    return {
        kumi,
        kumiById,
        dirs,
        kumisByDirId,

        itemById,

        init,

        createDir,
        deleteDir,
        rename,

        createKumi,
        deleteKumi,
        addKumi,

        addArtifact,
        deleteArtifact,
    }
}

const s = store()

export function watchContent() {
    return s.kumi.value
}

// watch(() => s.kumi.value, newValue => {
//     localStorage.setItem("kumi2", JSON.stringify(newValue))
// }, {
//     deep: true
// })

export function useKumiStore(): ReturnType<typeof store> {
    return s
}
