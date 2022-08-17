import {ArtifactMainStatName, type ArtifactPosition, ArtifactStatName, ArtifactSubStatName} from "@/types/artifact"

export const artifactEff: Record<number, Record<ArtifactSubStatName, number[]>> = {
    "4": {
        critical: [0.022, 0.025, 0.028, 0.031],
        lifePercentage: [0.033, 0.037, 0.042, 0.047],
        attackPercentage: [0.033, 0.037, 0.042, 0.047],
        recharge: [0.036, 0.041, 0.047, 0.052],
        defendPercentage: [0.041, 0.047, 0.053, 0.058],
        criticalDamage: [0.044, 0.05, 0.056, 0.062],
        attackStatic: [11, 12, 14, 16],
        defendStatic: [13, 15, 17, 19],
        elementalMastery: [13, 15, 17, 19],
        lifeStatic: [167, 191, 215, 239],
    },
    "5": {
        critical: [0.027, 0.031, 0.035, 0.039],
        lifePercentage: [0.041, 0.047, 0.053, 0.058],
        attackPercentage: [0.041, 0.047, 0.053, 0.058],
        recharge: [0.045, 0.052, 0.058, 0.065],
        defendPercentage: [0.051, 0.058, 0.066, 0.073],
        criticalDamage: [0.054, 0.062, 0.07, 0.078],
        attackStatic: [14, 16, 18, 19],
        defendStatic: [16, 19, 21, 23],
        elementalMastery: [16, 19, 21, 23],
        lifeStatic: [209, 239, 269, 299],
    }
}

interface ArtifactTagData {
    name: ArtifactStatName,
    chs: string,
    percentage: boolean,
    max: {
        4: number,
        5: number
    }
}

export const artifactTags: Record<ArtifactStatName, ArtifactTagData> = {
    "cureEffect": {
        "name": "cureEffect",
        "chs": "治疗加成",
        "percentage": true,
        "max": {
            "4": 0.268,
            "5": 0.359
        },
    },
    "criticalDamage": {
        "name": "criticalDamage",
        "chs": "暴击伤害",
        "percentage": true,
        "max": {
            "4": 0.464,
            "5": 0.622
        }
    },
    "critical": {
        "name": "critical",
        "chs": "暴击率",
        "percentage": true,
        "max": {
            "4": 0.232,
            "5": 0.311
        }
    },
    "attackStatic": {
        "name": "attackStatic",
        "chs": "攻击力",
        "percentage": false,
        "max": {
            "4": 232,
            "5": 311
        }
    },
    "attackPercentage": {
        "name": "attackPercentage",
        "chs": "攻击力%",
        "percentage": true,
        "max": {
            "4": 0.348,
            "5": 0.466
        }
    },
    "elementalMastery": {
        "name": "elementalMastery",
        "chs": "元素精通",
        "percentage": false,
        "max": {
            "4": 139,
            "5": 187
        }
    },
    "recharge": {
        "name": "recharge",
        "chs": "元素充能效率",
        "percentage": true,
        "max": {
            "4": 0.387,
            "5": 0.518
        }
    },
    "lifePercentage": {
        "name": "lifePercentage",
        "chs": "生命值%",
        "percentage": true,
        "max": {
            "4": 0.348,
            "5": 0.466
        }
    },
    "defendPercentage": {
        "name": "defendPercentage",
        "chs": "防御力%",
        "percentage": true,
        "max": {
            "4": 0.435,
            "5": 0.583
        }
    },
    "lifeStatic": {
        "name": "lifeStatic",
        "chs": "生命值",
        "percentage": false,
        "max": {
            "4": 3571,
            "5": 4780
        }
    },
    "defendStatic": {
        "name": "defendStatic",
        "chs": "防御力",
        "percentage": false,
        "max": {
            "4": 19,
            "5": 23
        }
    },
    "thunderBonus": {
        "name": "thunderBonus",
        "chs": "雷元素伤害加成",
        "percentage": true,
        "max": {
            "4": 0.348,
            "5": 0.466
        }
    },
    "fireBonus": {
        "name": "fireBonus",
        "chs": "火元素伤害加成",
        "percentage": true,
        "max": {
            "4": 0.348,
            "5": 0.466
        }
    },
    "waterBonus": {
        "name": "waterBonus",
        "chs": "水元素伤害加成",
        "percentage": true,
        "max": {
            "4": 0.348,
            "5": 0.466
        }
    },
    "iceBonus": {
        "name": "iceBonus",
        "chs": "冰元素伤害加成",
        "percentage": true,
        "max": {
            "4": 0.348,
            "5": 0.466
        }
    },
    "windBonus": {
        "name": "windBonus",
        "chs": "风元素伤害加成",
        "percentage": true,
        "max": {
            "4": 0.348,
            "5": 0.466
        }
    },
    "rockBonus": {
        "name": "rockBonus",
        "chs": "岩元素伤害加成",
        "percentage": true,
        "max": {
            "4": 0.348,
            "5": 0.466
        }
    },
    "physicalBonus": {
        "name": "physicalBonus",
        "chs": "物理伤害加成",
        "percentage": true,
        "max": {
            "4": 0.435,
            "5": 0.583
        }
    },
    "dendroBonus": {
        "name": "dendroBonus",
        "chs": "草元素伤害加成",
        "percentage": true,
        "max": {
            "4": 0.435,
            "5": 0.583
        }
    }
}

export const mainStatMap: Record<ArtifactPosition, ArtifactMainStatName[]> = {
    flower: ["lifeStatic"],
    feather: ["attackStatic"],
    sand: ["attackPercentage", "lifePercentage", "defendPercentage", "elementalMastery", "recharge"],
    cup: [
        "thunderBonus",
        "fireBonus",
        "waterBonus",
        "iceBonus",
        "windBonus",
        "rockBonus",
        "dendroBonus",
        "physicalBonus",
        "attackPercentage",
        "lifePercentage",
        "defendPercentage",
        "elementalMastery"
    ],
    head: [
        "critical",
        "criticalDamage",
        "attackPercentage",
        "cureEffect",
        "elementalMastery",
        "lifePercentage",
        "defendPercentage",
    ]
}

export const subStats: ArtifactSubStatName[] = [
    "critical",
    "criticalDamage",
    "attackStatic",
    "attackPercentage",
    "lifeStatic",
    "lifePercentage",
    "defendStatic",
    "defendPercentage",
    "elementalMastery",
    "recharge",
]

export const positions: ArtifactPosition[] = [
    "flower", "feather", "sand", "cup", "head"
]
