import badge from "@asset/badges/abeiduo.png";
import config from "./abeiduoDefaultConfig";

import skill from "./skill";

function f(config) {
    let isConste2 = config.cArgs.constellation >= 2;
    let eLevel = config.cArgs.skill2;
    let qLevel = config.cArgs.skill3;

    let eDmg1 = skill.e.dmg1[eLevel - 1];
    let eDmg2 = skill.e.dmg2[eLevel - 1];
    let qDmg1 = skill.q.dmg1[qLevel - 1];
    let qDmg2 = skill.q.dmg2[qLevel - 1];

    let eCount = config.tArgs.eCount;
    let qCount = config.tArgs.qCount;
    let qFreq = config.tArgs.qFreq;
    let c2Count = config.tArgs.c2Count;

    return function (attribute) {
        let def = attribute.defend();
        let atk = attribute.attack();

        let eCrit = Math.min(attribute.eCritical, 1);
        let eBonus = attribute.eBonus + attribute.bonus + attribute.rockBonus;
        let e
            = atk * eDmg1 * (1 + eBonus) * (1 + eCrit * attribute.criticalDamage)
            + def * eDmg2 * (1 + eBonus) * (1 + eCrit * attribute.criticalDamage) * eCount
        ;

        let qCrit = Math.min(attribute.qCritical, 1);
        let qBonus = attribute.qBonus + attribute.bonus + attribute.rockBonus;
        let q = atk * (1 + qBonus) * (1 + qCrit * attribute.criticalDamage) * (qDmg1 + qDmg2 * qCount);
        if (isConste2) {
            q += def * 0.3 * c2Count;
        }

        return e * (1 - qFreq) + q * qFreq;
    };
}

export default {
    name: "abeiduoDefault",
    chs: "阿贝多-白垩之子",
    description: [
        "兼顾E和Q",
        "2命加成算作两次计数",
        "仅供参考",
    ],
    tags: [
        "阿贝多",
        "输出",
    ],
    func: f,
    "for": "abeiduo",
    badge,
    needConfig: true,
    config,
}