import type {
    ArtifactPosition, ArtifactSetName, ArtifactSetNameWasm, ArtifactSlotNameWasm, ArtifactStatName,
    ArtifactStatNameWasm,
    ArtifactStatWasm,
    IArtifact,
    IArtifactTag,
    IArtifactWasm
} from "@/types/artifact"

const nameMap: any = {
    "adventurer": "Adventurer",
    "archaicPetra": "ArchaicPetra",
    "berserker": "Berserker",
    "blizzardStrayer": "BlizzardStrayer",
    "bloodstainedChivalry": "BloodstainedChivalry",
    "braveHeart": "BraveHeart",
    "crimsonWitch": "CrimsonWitchOfFlames",
    "defenderWill": "DefendersWill",
    "emblemOfSeveredFate": "EmblemOfSeveredFate",
    "gambler": "Gambler",
    "gladiatorFinale": "GladiatorsFinale",
    "heartOfDepth": "HeartOfDepth",
    "huskOfOpulentDreams": "HuskOfOpulentDreams",
    "instructor": "Instructor",
    "lavaWalker": "Lavawalker",
    "luckyDog": "LuckyDog",
    "maidenBeloved": "MaidenBeloved",
    "martialArtist": "MartialArtist",
    "noblesseOblige": "NoblesseOblige",
    "oceanHuedClam": "OceanHuedClam",
    "paleFlame": "PaleFlame",
    "prayersForDestiny": "PrayersForDestiny",
    "prayersForIllumination": "PrayersForIllumination",
    "prayersForWisdom": "PrayersForWisdom",
    "prayersToSpringtime": "PrayersToSpringtime",
    "resolutionOfSojourner": "ResolutionOfSojourner",
    "retracingBolide": "RetracingBolide",
    "scholar": "Scholar",
    "shimenawaReminiscence": "ShimenawasReminiscence",
    "tenacityOfTheMillelith": "TenacityOfTheMillelith",
    "exile": "TheExile",
    "thunderingFury": "ThunderingFury",
    "thunderSmoother": "Thundersoother",
    "tinyMiracle": "TinyMiracle",
    "travelingDoctor": "TravelingDoctor",
    "viridescentVenerer": "ViridescentVenerer",
    "wandererTroupe": "WanderersTroupe",
}
Object.freeze(nameMap)

let nameMapReverse: any = {}
for (const key in nameMap) {
    const value = nameMap[key]
    nameMapReverse[value] = key
}
Object.freeze(nameMapReverse)

const slotMap: Record<ArtifactPosition, ArtifactSlotNameWasm> = {
    "flower": "Flower",
    "feather": "Feather",
    "sand": "Sand",
    "cup": "Goblet",
    "head": "Head",
}

const statNameMap: any = {
    "cureEffect": "HealingBonus",
    "lifeStatic": "HPFixed",
    "lifePercentage": "HPPercentage",
    "attackStatic": "ATKFixed",
    "attackPercentage": "ATKPercentage",
    "defendStatic": "DEFFixed",
    "defendPercentage": "DEFPercentage",
    "critical": "CriticalRate",
    "criticalDamage": "CriticalDamage",
    "elementalMastery": "ElementalMastery",
    "recharge": "Recharge",
    "thunderBonus": "ElectroBonus",
    "fireBonus": "PyroBonus",
    "waterBonus": "HydroBonus",
    "iceBonus": "CryoBonus",
    "windBonus": "AnemoBonus",
    "rockBonus": "GeoBonus",
    "physicalBonus": "PhysicalBonus"
}

let statNameMapReverse: any = {}
for (let key in statNameMap) {
    const value = statNameMap[key]
    statNameMapReverse[value] = key
}
Object.freeze(statNameMapReverse)
Object.freeze(statNameMap)

function convertStat(stat: IArtifactTag): ArtifactStatWasm {
    return [
        statNameMap[stat.name],
        stat.value,
    ]
}

export function convertArtifact(artifact: IArtifact): IArtifactWasm {
    return {
        "set_name": convertArtifactName(artifact.setName),
        "slot": slotMap[artifact.position],
        "level": artifact.level,
        "star": artifact.star,
        "main_stat": convertStat(artifact.mainTag),
        "sub_stats": artifact.normalTags.map(x => convertStat(x)),
        "id": artifact.id
    }
}

export function convertArtifactName(name: ArtifactSetName): ArtifactSetNameWasm {
    if (Object.prototype.hasOwnProperty.call(nameMap, name)) {
        return nameMap[name]
    } else {
        return name
    }
}

export function convertArtifactNameBack(name: ArtifactSetNameWasm): ArtifactSetName {
    if (Object.prototype.hasOwnProperty.call(nameMapReverse, name)) {
        return nameMapReverse[name]
    } else {
        return name
    }
}

// dir: 1: old -> new
export function convertArtifactStatName(name: ArtifactStatName): ArtifactStatNameWasm {
    return statNameMap[name]
}

export function convertArtifactStatNameBack(name: ArtifactStatNameWasm): ArtifactStatName {
    return statNameMapReverse[name]
}
