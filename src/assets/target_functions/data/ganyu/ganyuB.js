import badge from "./badge.png";

function f(config) {
    let isAmos = config.weapon.name === "amosizhigong";
    let hasTalent1 = config.character.hasTalent1;
    let hasTalent2 = config.character.hasTalent2;

    return function (attribute, context) {
        let attack = attribute.attack();
        let crit = attribute.bCritical;

        let bonus = attribute.bBonus + attribute.bonus + attribute.iceBonus;
        if (isAmos) {
            bonus += (config.weapon.refine * 0.02 + 0.06) * 3;
        }

        let isBS4 = (context.artifactSet.blizzardStrayer || 0) >= 4;
        if (isBS4) {
            // 4冰套折合20暴击率
            crit += 0.2
        }
        if (hasTalent1) {
            crit += 0.1;
        }
        if (hasTalent2) {
            bonus += 0.1;
        }

        return attack * (1 + bonus) * (1 + Math.min(crit, 1) * attribute.criticalDamage);
    };
}

export default {
    name: "ganyuB",
    chs: "甘雨-二段蓄力",
    description: [
        "靠二段蓄力输出",
        "4冰套折合20暴击率",
        "自动根据等级计算天赋，天赋1折合10%暴击率，天赋2折合10%伤害加成",
        "如果带了阿莫斯之弓，折合0.3秒buff"
    ],
    tags: [
        "输出",
        "甘雨",
    ],
    func: f,
    "for": "ganyu",
    badge,
    needConfig: true,
    needContext: true,
}