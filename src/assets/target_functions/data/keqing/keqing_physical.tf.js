import badge from "@asset/badges/keqing.png";
import config from "./KeqingPhysicalConfig";

function f(config) {
    let aFreq = config.tArgs.aFreq;
    let bFreq = config.tArgs.bFreq;
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

        let a = 1 + attribute.aBonus + commonBonus2;
        a = (critical * attribute.criticalDamage + 1) * a * 0.8522;

        let b = 1 + attribute.bBonus + commonBonus2;
        b = (bCritical * attribute.criticalDamage + 1) * b * 2.57;

        let e = 1 + attribute.eBonus + commonBonus;
        e = (eCritical * attribute.criticalDamage + 1) * e * 3.276;

        let q = 1 + attribute.qBonus + commonBonus;
        q = (qCritical * attribute.criticalDamage + 1) * q * 6.558;

        let expect = (a * aFreq + b * bFreq + e * eFreq + q * qFreq) * attack;

        return expect;
    }
}

export default {
    name: "keqingPhysical",
    chs: "刻晴-物晴",
    description: [
        "物理刻晴",
        "1段E",
    ],
    tags: [
        "输出",
        "刻晴",
    ],
    func: f,
    "for": "keqing",
    badge,
    needConfig: true,
    needContext: true,
    config,
}