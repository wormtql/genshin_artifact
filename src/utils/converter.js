const nameMap = {
    "adventurer": "Adventurer",
    "archaicpetra": "ArchaicPetra",
    "berserker": "Berserker",
    "blizzardstrayer": "BlizzardStrayer",
    "bloodstainedchivalry": "BloodstainedChivalry",
    "braveheart": "BraveHeart",
    "crimsonwitch": "CrimsonWitchOfFlames",
    "defenderwill": "DefendersWill",
    "emblemofseveredfate": "EmblemOfSeveredFate",
    "gambler": "Gambler",
    "gladiatorfinale": "GladiatorsFinale",
    "heartofdepth": "HeartOfDepth",
    "huskofopulentdreams": "HuskOfOpulentDreams",
    "instructor": "Instructor",
    "lavawalker": "Lavawalker",
    "luckydog": "LuckyDog",
    "maidenbeloved": "MaidenBeloved",
    "martialartist": "MartialArtist",
    "noblesseoblige": "NoblesseOblige",
    "oceanhuedclam": "OceanHuedClam",
    "paleflame": "PaleFlame",
    "prayersfordestiny": "PrayersForDestiny",
    "prayersforillumination": "PrayersForIllumination",
    "prayersforwisdom": "PrayersForWisdom",
    "prayerstospringtime": "PrayersToSpringtime",
    "resolutionofsojourner": "ResolutionOfSojourner",
    "retracingbolide": "RetracingBolide",
    "scholar": "Scholar",
    "shimenawareminiscence": "ShimenawasReminiscence",
    "tenacityofthemillelith": "TenacityOfTheMillelith",
    "exile": "TheExile",
    "thunderingfury": "ThunderingFury",
    "thundersmoother": "Thundersoother",
    "tinymiracle": "TinyMiracle",
    "travelingdoctor": "TravelingDoctor",
    "viridescentvenerer": "ViridescentVenerer",
    "wanderertroupe": "WanderersTroupe",
}

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
    const lower = name.toLowerCase()
    if (Object.prototype.hasOwnProperty.call(nameMap, lower)) {
        return nameMap[lower]
    } else {
        return name
    }
    // return nameMap[name.toLowerCase()]
}

// dir: 1: old -> new
export function convertArtifactStatName(name) {
    return statNameMap[name]
}
