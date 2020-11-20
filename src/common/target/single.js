import { all } from "./base";

function createSingle(property) {
    return function(attribute) {
        return {
            value: attribute[property]
        };
    }
}

function attack(attribute) {
    return { value: attribute.attack1 + attribute.attack2 + attribute.attack3 };
}

function defend(attribute) {
    return { value: attribute.defend1 + attribute.defend2 + attribute.defend3 };
}

function life(attribute) {
    return { value: attribute.life1 + attribute.life2 + attribute.life3 };
}

function expected(attribute) {
    const bonus = attribute.physicalBonus + attribute.bonus + attribute.aBonus;
    const attack = attribute.attack1 + attribute.attack2 + attribute.attack3;

    const a = Math.max(1, attribute.critical) * (1 + attribute.criticalDamage);
    const b = Math.max(1 - attribute.critical, 0);

    return {
        value: (a + b) * attack * (1 + bonus),
    }
}

function basicConfig(element) {
    return {
        element,
        aFreq: 80,
        aRatio: 0.1,
        aTimes: 0.9,
        bFreq: 0,
        bRatio: 0,
        bTimes: 0,
        eFreq: 10,
        eTimes: 2,
        qFreq: 10,
        qTimes: 7
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
    },
    {
        value: "thunder",
        name: "雷元素期望输出",
        calc: all(basicConfig("thunder")),
        description: "雷元素主C的期望输出，不苛刻的情况下可以选择次目标函数",
    },
    {
        value: "fire",
        name: "火元素期望输出",
        calc: all(basicConfig("fire")),
        description: "火元素主C的期望输出，不苛刻的情况下可以选择次目标函数",
    },
    {
        value: "water",
        name: "水元素期望输出",
        calc: all(basicConfig("water")),
        description: "水元素主C的期望输出，不苛刻的情况下可以选择次目标函数",
    },
    {
        value: "ice",
        name: "冰元素期望输出",
        calc: all(basicConfig("ice")),
        description: "冰元素主C的期望输出，不苛刻的情况下可以选择次目标函数",
    },
    {
        value: "wind",
        name: "风元素期望输出",
        calc: all(basicConfig("wind")),
        description: "风元素主C的期望输出，不苛刻的情况下可以选择次目标函数",
    },
    {
        value: "rock",
        name: "岩元素期望输出",
        calc: all(basicConfig("rock")),
        description: "岩元素主C的期望输出，不苛刻的情况下可以选择次目标函数",
    },
]