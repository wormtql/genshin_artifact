import type {IArtifactContentOnly} from "@/types/artifact"
// @ts-ignore
import objectHash from "object-hash"

// hash of an artifact, including only properties in game
export function hash(artifact: IArtifactContentOnly): string {
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

// hash with order of sub stats, without sub stats' value
export function hashExceptValue(artifact: IArtifactContentOnly): string {
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