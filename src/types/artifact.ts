export interface IArtifactTag {
    name: ArtifactStatName,
    value: number
}

export interface ArtifactMainStat {
    name: ArtifactMainStatName,
    value: number
}

export type ArtifactPosition = "flower" | "cup" | "feather" | "head" | "sand"

export interface IArtifact {
    setName: ArtifactSetName,
    position: ArtifactPosition,
    star: number,
    mainTag: IArtifactTag,
    normalTags: IArtifactTag[],
    level: number,

    omit: boolean,
    id: number,
    contentHash: string,
}

export interface IArtifactContentOnly {
    setName: string,
    position: ArtifactPosition,
    star: number,
    mainTag: IArtifactTag,
    normalTags: IArtifactTag[],
    level: number
}

export type ArtifactSetName = string

export type ArtifactStatName
    = "lifeStatic"
    | "attackStatic"
    | "attackPercentage"
    | "lifePercentage"
    | "defendPercentage"
    | "elementalMastery"
    | "recharge"
    | "thunderBonus"
    | "fireBonus"
    | "waterBonus"
    | "iceBonus"
    | "windBonus"
    | "rockBonus"
    | "physicalBonus"
    | "critical"
    | "criticalDamage"
    | "cureEffect"
    | "defendStatic"

export type ArtifactMainStatName
    = "lifeStatic"
    | "attackStatic"
    | "attackPercentage"
    | "lifePercentage"
    | "defendPercentage"
    | "elementalMastery"
    | "recharge"
    | "thunderBonus"
    | "fireBonus"
    | "waterBonus"
    | "iceBonus"
    | "windBonus"
    | "rockBonus"
    | "physicalBonus"
    | "critical"
    | "criticalDamage"
    | "cureEffect"

export type ArtifactSubStatName
    = "critical"
    | "criticalDamage"
    | "attackStatic"
    | "attackPercentage"
    | "lifeStatic"
    | "lifePercentage"
    | "defendStatic"
    | "defendPercentage"
    | "elementalMastery"
    | "recharge"
