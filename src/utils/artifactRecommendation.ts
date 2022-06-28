import type {IArtifact} from "@/types/artifact"
import {convertArtifact} from "@/utils/converter"
import {useArtifactStore} from "@/store/pinia/artifact"
import {usePresetStore} from "@/store/pinia/preset"
import {wasmGetArtifactsRankByCharacter} from "@/wasm"

const artifactStore = useArtifactStore()
const presetStore = usePresetStore()

// get artifact recommendation according to current character presets
// @return [[id, score]]
export async function getArtifactsRecommendation(): Promise<[number, number][]> {
    const artifacts: IArtifact[] = [...artifactStore.artifacts.value.values()]
    const artifacts0 = artifacts.filter(a => a.level === 0)
    const artifactsWasm = artifacts0.map(a => convertArtifact(a))

    const presetItems = presetStore.allFlat.value.map(e => e.item)
    let scores: Map<number, number> = new Map()
    for (let presetItem of presetItems) {
        const characterInterface = presetItem.character
        const weaponInterface = presetItem.weapon
        const tfInterface = presetItem.targetFunction

        const result = await wasmGetArtifactsRankByCharacter(characterInterface, weaponInterface, tfInterface, artifactsWasm)
        for (let item of result) {
            const [id, score] = item
            if (!scores.has(id)) {
                scores.set(id, 0)
            }
            scores.set(id, scores.get(id) as number + score)
        }
    }

    let temp: [number, number][] = []
    for (const id of scores.keys()) {
        temp.push([id, scores.get(id) as number])
    }
    // let temp = Object.keys(scores).map(id => [id, scores[id as any]])
    temp.sort((a, b) => b[1] - a[1])

    return temp
}