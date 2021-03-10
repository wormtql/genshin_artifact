import badge from "@asset/badges/keqing.png";

function f(config) {
    let isXiali = config.weapon.name === "xialilongyin";
    let refine = config.weapon.refine;

    return function (attribute) {
        let atk = attribute.attack();
        let crit = Math.min(1, attribute.qCritical + 0.15);

        let bonus = attribute.qBonus + attribute.thunderBonus + attribute.bonus;
        if (isXiali) {
            bonus += 0.04 * refine + 0.16;
        }
    
        return atk * (1 + bonus) * (1 + crit * attribute.criticalDamage);
    }
}


export default {
    name: "keqingQ",
    chs: "刻晴-天街巡游",
    description: [
        "使刻晴Q的伤害期望最大（考虑刻晴天赋15%暴击）",
        "由于Q挂雷很强，因此平雷套、匣里龙吟算满buff",
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
}