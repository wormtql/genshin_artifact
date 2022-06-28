import {type ICharacter} from "@/types/character"
import {type ITargetFunction} from "@/types/targetFunction"
import {type IWeapon} from "@/types/weapon"
import type {ArtifactStatName} from "@/types/artifact"

export interface IBuff {
    name: string,
    config: any,
    lock: boolean,
}

export type IBuffWithID = IBuff & { id: number }

export type IBuffWasm = Omit<IBuff, "lock">

export interface IPreset {
    name: string,
    algorithm?: PresetAlgorithm,
    artifactConfig?: any,
    artifactEffectMode?: ArtifactEffectMode,
    constraint?: IConstraint,
    dslSource?: string,
    useDSL?: boolean,
    filter?: IPresetArtifactFilter,
    character: ICharacter,
    weapon: IWeapon,
    targetFunction: ITargetFunction,
    buffs?: IBuff[]
}

export type ArtifactEffectMode = "custom" | "auto"

export type PresetAlgorithm = "AStar" | "Heuristic" | "Naive"

export interface IConstraint {
    minCritical: number,
    minCriticalDamage: number,
    minElementalMastery: number,
    minRecharge: number,
    setNames: string[]
}

export interface IPresetArtifactFilter {
    gobletMainStats: ArtifactStatName[],
    headMainStats: ArtifactStatName[],
    sandMainStats: ArtifactStatName[]
}