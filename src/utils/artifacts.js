import { positions } from "@const/misc"
import {artifactEff} from "@const/artifact"
import objectHash from "object-hash"
import store from "@/store/store"
import {artifactsData} from "@artifact"
import { toSnakeCase, deepCopy } from "@util/common"
import { wasmGetArtifactsRankByCharacter } from "@/wasm"
import {convertArtifact} from "@util/converter"


// count how many artifacts

// count min and max upgrade count
export function howManyUpgradeCount(value, tagName, star) {
    const eff = artifactEff[star][tagName]
    const min = Math.round(value / eff[3])
    const max = Math.round(value / eff[0])
    // if (tagName === "attackPercentage") {
    //     console.log(value);
    // }

    // value = Math.round(value * 1000);
    // let eff0 = Math.round(eff[0] * 1000);
    // let eff3 = Math.round(eff[3] * 1000);
    //
    // let max = Math.floor(value / eff0);
    // let min = Math.ceil(value / eff3);
    return [min, max];
}

export function hash(artifact) {
    // return objectHash(artifact, {
    //     excludeKeys: (k) => {
    //         return k === "id" || k === "omit" || k === "detailName";
    //     }
    // });

    let subStatNames = ""
    let subStatValues = ""
    for (let stat of artifact.normalTags) {
        subStatNames += stat.name
        subStatValues += stat.value.toFixed(5)
    }

    const object = {
        name: artifact.setName,
        position: artifact.position,
        star: artifact.star,
        level: artifact.level,
        mainStatName: artifact.mainTag.name,
        mainStatValue: artifact.mainTag.value.toFixed(5),
        subStatNames,
        subStatValues
    }

    return objectHash(object)
}

export function hashExceptValue(artifact) {
    let subStatNames = ""
    for (let stat of artifact.normalTags) {
        subStatNames += stat.name
    }

    const object = {
        name: artifact.setName,
        position: artifact.position,
        star: artifact.star,
        mainStatName: artifact.mainTag.name,
        subStatNames
    }

    return objectHash(object)
}

export function newDefaultArtifactConfigForWasm() {
    let configs = {}

    for (let name in artifactsData) {
        const data = artifactsData[name]
        const name2 = data.name2
        const config4 = data.config4 ?? []
        if (config4.length > 0) {
            let c = {}
            for (let item of config4) {
                c[item.name] = item.default
            }

            const snake = toSnakeCase(name2)
            const configName = "config_" + snake
            configs[configName] = c
        }
    }
    // console.log(configs)

    return configs

    // return {
    //     "config_archaic_petra": {
    //         "element": "Electro",
    //         rate: 0,
    //     },
    //     "config_berserker": { rate: 0 },
    //     "config_blizzard_strayer": { critical_bonus: 0 },
    //     "config_bloodstained_chivalry": { rate: 0 },
    //     "config_brave_heart": { rate: 0 },
    //     "config_crimson_witch_of_flames": { level: 0 },
    //     "config_heart_of_depth": { rate: 0 },
    //     "config_husk_of_opulent_dreams": { level: 0 },
    //     "config_instructor": { rate: 0 },
    //     "config_lavawalker": { rate: 0 },
    //     "config_martial_artist": { rate: 0 },
    //     "config_noblesse_oblige": { rate: 0 },
    //     "config_pale_flame": { avg_level: 0, full_rate: 0 },
    //     "config_retracing_bolide": { rate: 0 },
    //     "config_shimenawas_reminiscence": { rate: 0 },
    //     "config_tenacity_of_the_millelith": { rate: 0 },
    //     "config_thundersoother": { rate: 0 },
    // }
}

export function toggleArtifact(id) {
    store.commit("artifacts/toggleArtifactById", {
        id
    })
}

export function removeArtifact(id) {
    store.commit("artifacts/removeArtifactById", {
        id
    })
}

export function getArtifact(id) {
    const byId = store.getters["artifacts/artifactsById"]
    return byId[id]
}

export function getArtifactImage(setName, position) {
    const data = artifactsData[setName]
    if (data[position]) {
        return data[position].url
    }
    throw new Error("artifact can't exist")
}

export function getArtifactImageByArtifact(artifact) {
    return getArtifactImage(artifact.setName, artifact.position)
}

export function updateArtifact(id, newArtifact) {
    store.commit("artifacts/updateArtifact", { id, artifact: newArtifact })
}

export function newArtifact(artifact) {
    store.commit("artifacts/addArtifactV2", {
        artifact
    })
}

export async function importMonaJson(rawObj, removeNonExisting) {
    let hashAll = {}
    let hashEV = {}
    let existingIds = new Set()

    let artifactsFlat = store.getters["artifacts/allFlat"]

    for (let artifact of artifactsFlat) {
        const h = hash(artifact)
        const hev = hashExceptValue(artifact)

        hashAll[h] = artifact
        hashEV[hev] = artifact
    }

    let skipCount = 0
    let upgradeCount = 0
    let newCount = 0

    let importFlat = [].concat(rawObj.flower ?? []).concat(rawObj.feather ?? []).concat(rawObj.sand ?? []).concat(rawObj.cup ?? []).concat(rawObj.head ?? [])
    for (let artifact of importFlat) {
        const h = hash(artifact)
        const hev = hashExceptValue(artifact)

        if (hashAll[h]) {
            // this artifacts exists
            const id = hashAll[h].id
            skipCount += 1
            existingIds.add(id)
            continue
        }

        if (hashEV[hev] && artifact.level > hashEV[hev].level) {
            // this artifacts is upgraded
            console.log("upgrade")
            console.log("old", JSON.stringify(hashEV[hev]))
            console.log("new", JSON.stringify(artifact))
            const id = hashEV[hev].id
            updateArtifact(id, artifact)
            upgradeCount += 1
            existingIds.add(id)
            continue
        }

        // new artifact
        newCount += 1
        newArtifact(artifact)
    }

    let removeCount = 0
    if (removeNonExisting) {
        for (let originalArtifacts of Object.values(hashAll)) {
            const id = originalArtifacts.id
            if (!existingIds.has(id)) {
                removeCount += 1
                console.log("remove", originalArtifacts)
                removeArtifact(id)
            }
        }
    }

    console.log(`import result: skip${skipCount}, upgrade${upgradeCount}, new${newCount}, remove${removeCount}`)
}

export function getArtifactThumbnail(name) {
    const data = artifactsData[name]

    for (let position in positions) {
        if (Object.prototype.hasOwnProperty.call(data, position)) {
            return data[position].url
        }
    }

    throw new Error("artifact with no artifact")
}

export function upgradeArtifactConfig(oldConfig) {
    if (!oldConfig) {
        return newDefaultArtifactConfigForWasm()
    }

    let newConfig = {}
    for (let name in artifactsData) {
        const data = artifactsData[name]
        const name2 = data.name2
        const snake = toSnakeCase(name2)
        const configName = "config_" + snake

        if (Object.prototype.hasOwnProperty.call(oldConfig, configName)) {
            newConfig[configName] = deepCopy(oldConfig[configName])
        } else {
            const config4 = data.config4 ?? []
            if (config4.length > 0) {
                let c = {}
                for (let item of config4) {
                    c[item.name] = item.default
                }

                newConfig[configName] = c
            }
        }
    }

    return newConfig
}

export async function getArtifactsRecommendation() {
    const artifactsAll = store.getters["artifacts/allFlat"]
    const artifacts0 = artifactsAll.filter(a => a.level === 0)
    const artifactsWasm = artifacts0.map(a => convertArtifact(a))

    const presetItems = store.getters["presets/allFlat"].map(e => e.item).filter(e => e)

    let scores = {}
    for (let presetItem of presetItems) {
        const characterInterface = presetItem.character
        const weaponInterface = presetItem.weapon
        const tfInterface = presetItem.targetFunction

        const result = await wasmGetArtifactsRankByCharacter(characterInterface, weaponInterface, tfInterface, artifactsWasm)
        for (let item of result) {
            const [id, score] = item
            if (!Object.prototype.hasOwnProperty.call(scores, id)) {
                scores[id] = 0
            }
            scores[id] += score
        }
    }

    let temp = Object.keys(scores).map(id => [id, scores[id]])
    temp.sort((a, b) => b[1] - a[1])

    return temp
}
