function createSingle(property) {
    return function(attribute) {
        return {
            value: attribute[property]
        };
    }
}

function attack(attribute) {
    return { value: attribute.attack1 + attribute.attack2 };
}

function defend(attribute) {
    return { value: attribute.defend1 + attribute.defend2 };
}

function life(attribute) {
    return { value: attribute.life1 + attribute.life2 };
}

function expected(attribute) {
    const bonus = attribute.physicalBonus + attribute.bonus + attribute.aBonus;

    const a = Math.max(1, attribute.critical) * (1 + attribute.criticalDamage);
    const b = Math.max(1 - attribute.critical, 0);

    return {
        value: (a + b) * (attribute.attack1 + attribute.attack2) * (1 + bonus)
    }
}

export const plans = [
    {
        value: "attack",
        name: "攻击力",
        calc: attack,
        description: "总攻击力",
    },
    {
        value: "defend",
        name: "防御力",
        calc: defend,
        description: "总防御力",
    },
    {
        value: "life",
        name: "生命值",
        calc: life,
        description: "总生命值"
    },
    {
        value: "critical",
        name: "平A暴击率",
        calc: createSingle("critical"),
        description: "暴击率",
    },
    {
        value: "bCritical",
        name: "重击暴击率",
        calc: createSingle("bCritical"),
        description: "重击暴击率",
    },
    {
        value: "criticalDamage",
        name: "暴击伤害",
        calc: createSingle("criticalDamage"),
        description: "暴击伤害",
    },
    {
        value: "elementalMastery",
        name: "元素精通",
        calc: createSingle("elementalMastery"),
        description: "元素精通",
    },
    {
        value: "recharge",
        name: "元素充能效率",
        calc: createSingle("recharge"),
        description: "元素充能效率",
    },
    {
        value: "plainA",
        name: "平A期望输出",
        calc: expected,
        description: "平A的期望输出（不考虑防御力），选不出目标函数时可以用这个",
        tags: [
            "仅考虑物理伤害",
            "考虑暴击和爆伤"
        ]
    }
]