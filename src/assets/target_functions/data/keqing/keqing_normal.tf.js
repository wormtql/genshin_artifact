import badge from "@asset/badges/keqing.png";
import config from "./KeqingNormalConfig";

function f(config) {
    let aFreq = config.tArgs.aFreq;
    let aEle = config.tArgs.aEle;
    let bFreq = config.tArgs.bFreq;
    let bEle = config.tArgs.bEle;
    let eFreq = config.tArgs.eFreq;
    let qFreq = config.tArgs.qFreq;

    let thunderFreq = config.tArgs.thunderFreq;

    let isXiali = config.weapon.name === "xialilongyin";
    let xialiBonus = isXiali ? (config.weapon.refine * 0.04 + 0.16) : 0;

    return function (attribute, context) {
        let isTS4 = (context.artifactSet.thunderSmoother || 0) >= 4;

        let otherBonus = (isTS4 ? (thunderFreq * 0.35) : 0) + xialiBonus * thunderFreq;

        const attack = attribute.attack();
        const critical = Math.min(attribute.critical, 1);
        const bCritical = Math.min(attribute.bCritical, 1);
        const eCritical = Math.min(attribute.eCritical, 1);
        const qCritical = Math.min(attribute.qCritical, 1);

        const commonBonus = attribute.thunderBonus + attribute.bonus + otherBonus;
        const commonBonus2 = attribute.physicalBonus + attribute.bonus;

        let a 
            = (1 - aEle) * (1 + attribute.aBonus + commonBonus2)
            + aEle * (1 + attribute.aBonus + commonBonus)
        ;
        a = (critical * attribute.criticalDamage + 1) * a * 0.8522;

        let b
            = (1 - bEle) * (1 + attribute.bBonus + commonBonus2)
            + bEle * (1 + attribute.bBonus + commonBonus)
        ;
        b = (bCritical * attribute.criticalDamage + 1) * b * 2.57;

        let e = (1 + attribute.eBonus + commonBonus);
        e = (eCritical * attribute.criticalDamage + 1) * e * 3.276;

        let q = (1 + attribute.qBonus + commonBonus);
        q = (qCritical * attribute.criticalDamage + 1) * q * 6.558;

        let expect = (a * aFreq + b * bFreq + e * eFreq + q * qFreq) * attack;

        return expect;
    }
}

export default {
    name: "keqingNormal",
    chs: "刻晴-斩尽牛杂",
    description: [
        "普通雷伤刻晴",
        "2段E",
    ],
    tags: [
        "输出",
        "刻晴",
        "元素伤害",
    ],
    func: f,
    "for": "keqing",
    badge,
    needConfig: true,
    needContext: true,
    config,
}