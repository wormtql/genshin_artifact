const nameMap = {
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

let nameMapReverse = {}
for (const key of Object.keys(nameMap)) {
    const value = nameMap[key]
    nameMapReverse[value] = key
}
Object.freeze(nameMapReverse)

const slotMap = {
    "flower": "Flower",
    "feather": "Feather",
    "sand": "Sand",
    "cup": "Goblet",
    "head": "Head",
}

const statNameMap = {
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

function convertStat(stat) {
    return [
        statNameMap[stat.name],
        stat.value,
    ]
}

export function convertArtifact(artifact) {
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

export function convertArtifactName(name) {
    if (Object.prototype.hasOwnProperty.call(nameMap, name)) {
        return nameMap[name]
    } else {
        return name
    }
    // return nameMap[name.toLowerCase()]
}

export function convertArtifactNameBack(name) {
    if (Object.prototype.hasOwnProperty.call(nameMapReverse, name)) {
        return nameMapReverse[name]
    } else {
        return name
    }
}

// dir: 1: old -> new
export function convertArtifactStatName(name) {
    return statNameMap[name]
}
