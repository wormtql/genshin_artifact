export interface IArtifactTag {
    name: ArtifactStatName,
    value: number
}

export interface IArtifactSubTag {
    name: ArtifactSubStatName,
    value: number,
}

export interface IArtifactMainTag {
    name: ArtifactMainStatName,
    value: number
}

export type ArtifactStatWasm = [ArtifactStatNameWasm, number]

export interface ArtifactMainStat {
    name: ArtifactMainStatName,
    value: number
}

export type ArtifactPosition = "flower" | "cup" | "feather" | "head" | "sand"

export interface IArtifact {
    setName: ArtifactSetName,
    position: ArtifactPosition,
    star: number,
    mainTag: IArtifactMainTag,
    normalTags: IArtifactSubTag[],
    level: number,

    omit: boolean,
    id: number,
    contentHash: string,
}

export interface IArtifactWasm {
    set_name: ArtifactSetNameWasm,
    slot: ArtifactSlotNameWasm,
    level: number,
    star: number,
    main_stat: [ArtifactStatNameWasm, number],
    sub_stats: [ArtifactStatNameWasm, number][],
    id: number
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

export type ArtifactSetNameWasm = string

export type ArtifactSlotNameWasm = "Flower" | "Feather" | "Sand" | "Goblet" | "Head"

export type ArtifactStatName
    = "lifeStatic"
    | "attackStatic"
    | "attackPercentage"
    | "lifePercentage"
    | "defendPercentage"
    | "elementalMastery"
    | "recharge"
    | "dendroBonus"
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

export type ArtifactStatNameWasm
    = "HealingBonus"
    | "HPFixed"
    | "HPPercentage"
    | "ATKFixed"
    | "ATKPercentage"
    | "DEFFixed"
    | "DEFPercentage"
    | "CriticalRate"
    | "CriticalDamage"
    | "ElementalMastery"
    | "Recharge"
    | "DendroBonus"
    | "ElectroBonus"
    | "PyroBonus"
    | "HydroBonus"
    | "CryoBonus"
    | "AnemoBonus"
    | "GeoBonus"
    | "PhysicalBonus"

export type ArtifactMainStatName
    = "lifeStatic"
    | "attackStatic"
    | "attackPercentage"
    | "lifePercentage"
    | "defendPercentage"
    | "elementalMastery"
    | "recharge"
    | "dendroBonus"
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
