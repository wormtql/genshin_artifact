import {defineStore} from "pinia"
import {reactive, ref, type Ref, watch} from "vue"
import {RandomIDProvider} from "@/utils/idProvider"


import {hash as hashArtifact} from "@/utils/artifactHash"
import {deepCopy} from "@/utils/common"
import {type ArtifactPosition, type IArtifact, type IArtifactContentOnly} from "@/types/artifact"

// let localStoredArtifacts = localStorage.getItem("artifacts");
// if (localStoredArtifacts) {
//     let obj: any = JSON.parse(localStoredArtifacts);

//     flower = obj.flower || [];
//     feather = obj.feather || [];
//     sand = obj.sand || [];
//     cup = obj.cup || [];
//     head = obj.head || [];

//     let temp = flower.concat(feather).concat(sand).concat(cup).concat(head);

//     for (let item of temp) {
//         if (!Object.prototype.hasOwnProperty.call(item, "id")) {
//             item.id = idProvider.generateId()
//         }
//         if (!Object.prototype.hasOwnProperty.call(item, "contentHash")) {
//             item.contentHash = hashArtifact(item)
//         }
//     }
// }

const f = () => {
    const artifacts: Ref<Map<number, IArtifact>> = ref(new Map())
    const hashes: Map<string, number> = reactive(new Map())

    const idProvider = new RandomIDProvider()

    const incHash = (key: string, value: number): void => {
        if (hashes.has(key)) {
            let newValue = hashes.get(key)! + value
            if (newValue > 0) {
                hashes.set(key, newValue)
            } else {
                hashes.delete(key)
            }
        } else {
            if (value > 0) {
                hashes.set(key, value)
            }
        }
    }

    function init(payload: any) {
        let flower: IArtifact[] = [];
        let feather: IArtifact[] = [];
        let sand: IArtifact[] = [];
        let cup: IArtifact[] = [];
        let head: IArtifact[] = [];

        if (payload) {
            flower = payload.flower || [];
            feather = payload.feather || [];
            sand = payload.sand || [];
            cup = payload.cup || [];
            head = payload.head || [];

            let temp = flower.concat(feather).concat(sand).concat(cup).concat(head);

            for (let item of temp) {
                if (!Object.prototype.hasOwnProperty.call(item, "id")) {
                    item.id = idProvider.generateId()
                }
                if (!Object.prototype.hasOwnProperty.call(item, "contentHash")) {
                    item.contentHash = hashArtifact(item)
                }
            }
        }

        artifacts.value.clear()
        hashes.clear()
        const temp = flower.concat(feather).concat(sand).concat(cup).concat(head)
        for (const item of temp) {
            artifacts.value.set(item.id, item)
            const hash = item.contentHash
            incHash(hash, 1)
        }
    }


    function getArtifact(id: number): IArtifact | undefined {
        return artifacts.value.get(id)
    }

    function removeArtifact(id: number) {
        const a = artifacts.value.get(id)
        if (a) {
            const hash = a.contentHash
            incHash(hash, -1)
        }
        artifacts.value.delete(id)
    }

    function addArtifact(artifact: IArtifactContentOnly, omit: boolean = false): number {
        let a = deepCopy(artifact) as IArtifact
        a.omit = omit
        a.id = idProvider.generateId()
        a.contentHash = hashArtifact(artifact)

        incHash(a.contentHash, 1)
        artifacts.value.set(a.id, a)

        return a.id
    }

    function unlockAll() {
        for (let a of artifacts.value.values()) {
            a.omit = false
        }
    }

    function deleteAll() {
        artifacts.value.clear()
    }

    function toggleArtifact(id: number) {
        let a = artifacts.value.get(id)
        if (a) {
            a.omit = !a.omit
        }
    }

    function lockArtifact(id: number) {
        let a = artifacts.value.get(id)
        if (a) {
            a.omit = true
        }
    }

    function unlockArtifact(id: number) {
        let a = artifacts.value.get(id)
        if (a) {
            a.omit = false
        }
    }

    function updateArtifact(id: number, newArtifact: IArtifactContentOnly) {
        let original = artifacts.value.get(id)
        if (original) {
            const oldHash = original.contentHash
            incHash(oldHash, -1)
            Object.assign(original, newArtifact)
            original.contentHash = hashArtifact(original)
            const newHash = original.contentHash
            incHash(newHash, 1)
        }
    }

    function isHashExists(hash: string): boolean {
        return hashes.has(hash)
    }

    const artifactsByPosition = computed((): Record<ArtifactPosition, IArtifact[]> => {
        let result = {
            "flower": [],
            "feather": [],
            "sand": [],
            "cup": [],
            "head": []
        } as Record<ArtifactPosition, IArtifact[]>;

        for (let item of artifacts.value.values()) {
            const position = item.position
            result[position].push(item)
        }

        return result
    })

    const artifactsCount = computed((): number => {
        return artifacts.value.size
    })

    const artifacts20Count = computed(() => {
        let count = 0
        for (const a of artifacts.value.values()) {
            if (a.level === 20) {
                 count += 1
            }
        }
        return count
    })

    return {
        artifacts,
        artifactsByPosition,
        artifactsCount,
        artifacts20Count,

        init,
        removeArtifact,
        addArtifact,
        unlockAll,
        deleteAll,
        toggleArtifact,
        lockArtifact,
        unlockArtifact,
        updateArtifact,
        getArtifact,

        isHashExists
    }
}

const s = f()

// watch(() => {
//     return {
//         flower: s.artifactsByPosition.value["flower"],
//         feather: s.artifactsByPosition.value["feather"],
//         sand: s.artifactsByPosition.value["sand"],
//         cup: s.artifactsByPosition.value["cup"],
//         head: s.artifactsByPosition.value["head"]
//     }
// }, newValue => {
//     localStorage.setItem("artifacts", JSON.stringify(newValue))
// }, {
//     deep: true
// })

export function watchContent() {
    return {
        flower: s.artifactsByPosition.value["flower"],
        feather: s.artifactsByPosition.value["feather"],
        sand: s.artifactsByPosition.value["sand"],
        cup: s.artifactsByPosition.value["cup"],
        head: s.artifactsByPosition.value["head"]
    }
}

export const useArtifactStore = () => {
    return s
}

// export const useArtifactStore = defineStore("artifact", () => {
//
// })
