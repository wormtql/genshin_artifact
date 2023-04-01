import {artifactEff, artifactTags} from "@/constants/artifact"
// @ts-ignore
import objectHash from "object-hash"
import {artifactsData} from "@/assets/artifacts"
import { toSnakeCase, deepCopy } from "@/utils/common"
import { wasmGetArtifactsRankByCharacter } from "@/wasm"
import {convertArtifact, convertArtifactStatNameBack} from "@/utils/converter"
import type {
    ArtifactPosition,
    ArtifactSetName, ArtifactStatName,
    ArtifactSubStatName,
    IArtifact,
    IArtifactContentOnly
} from "@/types/artifact"
import { useArtifactStore } from "@/store/pinia/artifact"
import { hash, hashExceptValue } from "@/utils/artifactHash"
import { positions } from "@/constants/artifact"
import {useI18n} from "@/i18n/i18n";
import {useKumiStore} from "@/store/pinia/kumi";


const artifactStore = useArtifactStore()

// count min and max upgrade count
export function howManyUpgradeCount(value: number, tagName: ArtifactStatName, star: number): [number, number] {
    const eff = (artifactEff as any)[star][tagName]
    const min = Math.round(value / eff[3])
    const max = Math.round(value / eff[0])

    return [min, max];
}


// create new default artifact config
export function newDefaultArtifactConfigForWasm(): any {
    let configs: any = {}

    for (let name in artifactsData) {
        const data = artifactsData[name]
        const name2 = data.name2
        const config4 = data.config4 ?? []
        if (config4.length > 0) {
            let c: any = {}
            for (let item of config4) {
                c[item.name] = item.default
            }

            const snake = toSnakeCase(name2)
            const configName = "config_" + snake
            configs[configName] = c
        }
    }

    return configs
}

// toggle artifact omit/not omit
export function toggleArtifact(id: number) {
    artifactStore.toggleArtifact(id)
}

// remove artifact
export function removeArtifact(id: number) {
    artifactStore.removeArtifact(id)
}

// get artifact item
export function getArtifact(id: number): IArtifact | undefined {
    return artifactStore.artifacts.value.get(id)
}

// get image url
export function getArtifactImage(setName: ArtifactSetName, position: ArtifactPosition): string {
    const data = artifactsData[setName]
    if (data[position]) {
        return data[position].url
    }
    throw new Error("artifact can't exist")
}

export function getArtifactImageByArtifact(artifact: IArtifactContentOnly): string {
    return getArtifactImage(artifact.setName, artifact.position)
}

export function updateArtifact(id: number, newArtifact: IArtifactContentOnly): void {
    artifactStore.updateArtifact(id, newArtifact)
}

export function newArtifact(artifact: IArtifactContentOnly, omit: boolean = false): number {
    return artifactStore.addArtifact(artifact, omit)
}


interface ImportJsonResult {
    skip: number,
    upgrade: number,
    remove: number,
    add: number,
}

export function importMonaJson(rawObj: any, removeNonExisting: boolean, backupImportDir: boolean): ImportJsonResult {
    // hash of level, main stat, sub stats, rarity, set name, slot
    let hashAll: Record<string, IArtifact> = {}
    // hash of level, main stat without value, sub stats without value, rarity, set name, slot
    let hashEV: Record<string, IArtifact> = {}
    let existingIds = new Set()

    let equips: Map<string, number[]> = new Map()

    for (let artifact of artifactStore.artifacts.value.values()) {
        const h = hash(artifact)
        const hev = hashExceptValue(artifact)

        hashAll[h] = artifact
        hashEV[hev] = artifact
    }

    let skipCount = 0
    let upgradeCount = 0
    let newCount = 0

    let importFlat: any[] = [].concat(rawObj.flower ?? []).concat(rawObj.feather ?? []).concat(rawObj.sand ?? []).concat(rawObj.cup ?? []).concat(rawObj.head ?? [])
    for (let artifact of importFlat) {
        const h = hash(artifact)
        const hev = hashExceptValue(artifact)
        let artifactId = 0

        if (hashAll[h]) {
            // this artifacts exists
            const id = hashAll[h].id
            skipCount += 1
            existingIds.add(id)
            artifactId = id
        } else if (hashEV[hev] && artifact.level > hashEV[hev].level) {
            // this artifacts is upgraded
            // console.log("upgrade")
            // console.log("old", JSON.stringify(hashEV[hev]))
            // console.log("new", JSON.stringify(artifact))
            const id = hashEV[hev].id
            updateArtifact(id, artifact)
            upgradeCount += 1
            existingIds.add(id)
            artifactId = id
        } else {
            // new artifact
            newCount += 1
            artifactId = newArtifact(artifact, !!artifact.omit)
        }

        if (artifact.equip && artifact.equip !== "") {
            // artifact has equip data
            const equipCharacter = artifact.equip
            let arr = equips.get(equipCharacter)
            if (arr === undefined) {
                arr = []
                equips.set(equipCharacter, arr)
            }
            arr.push(artifactId)
        }
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

    // add new artifacts groups to store
    const kumiStore = useKumiStore()
    if(kumiStore.itemById(1)?.dir)
    {
        if(backupImportDir) kumiStore.backupImportDir()
        else kumiStore.clearDir(1)
        for (const equipName of equips.keys()) {
           const artifacts = equips.get(equipName)
           if (artifacts !== undefined) {
               kumiStore.addKumi(1, equipName, artifacts)
           }
        }   
    }

    return {
        skip: skipCount,
        upgrade: upgradeCount,
        add: newCount,
        remove: removeCount
    }
}

export function getArtifactThumbnail(name: ArtifactSetName): string {
    let data = artifactsData[name]
    if (!data) {
        console.log(name)
    }

    for (let position of positions) {
        if (Object.prototype.hasOwnProperty.call(data, position)) {
            return data[position].url
        }
    }

    throw new Error("artifact with no artifact")
}

// as artifact set number will increase, old config is not enough
// this function automatically upgrade old config to new config
// if new config key also exists in old config, use old value
// otherwise, use default value
export function upgradeArtifactConfig(oldConfig: any) {
    if (!oldConfig) {
        return newDefaultArtifactConfigForWasm()
    }

    let newConfig: any = {}

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
                let c: any = {}
                for (let item of config4) {
                    c[item.name] = item.default
                }

                newConfig[configName] = c
            }
        }
    }

    return newConfig
}

// get all artifacts(including omitted) using wasm format
export function getArtifactsWasm() {
    // const allFlat = store.getters["artifacts/allFlat"]

    let results: any[] = []
    for (let a of artifactStore.artifacts.value.values()) {
        results.push(convertArtifact(a))
    }
    return results
}

export function isArtifactExists(artifact: IArtifactContentOnly): boolean {
    const h = hash(artifact)
    return artifactStore.isHashExists(h)
}

/**
 * attackPercentage, 0.2 => "攻击力+20%"
 * attackStatic, 20 => "攻击力+20"
 */
export function displayedTag(name: ArtifactStatName, value: number) {
    const { t } = useI18n()

    let tagData = artifactTags[name];
    if (!tagData) {
        console.log(name)
        throw "tag name not exist";
    }

    let left = "";
    switch (name) {
        case "attackPercentage":
        case "attackStatic":
            // left = "攻击力";
            left = t("stat.attackStatic")
            break;
        case "lifePercentage":
        case "lifeStatic":
            // left = "生命值";
            left = t("stat.lifeStatic")
            break;
        case "defendPercentage":
        case "defendStatic":
            // left = "防御力";
            left = t("stat.defendStatic")
            break;
        default:
            // left = tagData.chs;
            left = t("stat", name)
            break;
    }

    if (tagData.percentage) {
        let s = (value * 100).toFixed(1);
        return left + "+" + s + "%";
    } else {
        return left + "+" + value;
    }
}

export function positionToIndex(p: ArtifactPosition): number {
    switch (p) {
        case "flower": return 0
        case "feather": return 1
        case "sand": return 2
        case "cup": return 3
        case "head": return 4
    }
}

export function defaultArtifactSortFunction(a: IArtifact, b: IArtifact): number {
    if (a.level !== b.level) {
        return b.level - a.level
    } else if (a.star !== b.star) {
        return b.star - a.star
    } else {
        return a.setName.localeCompare(b.setName)
    }
}

export function statName2Locale(name: ArtifactStatName): string {
    let data = artifactTags[name]
    if (!data) {
        const name2 = convertArtifactStatNameBack(name as any)
        data = artifactTags[name2]
    }

    if (!data) {
        throw new Error("cannot find name " + name)
    }

    const { t } = useI18n()

    // return data.chs
    return t("stat", data.name)
}