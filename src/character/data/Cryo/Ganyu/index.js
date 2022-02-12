import card from "./card.jpg"
import avatar from "@asset/badges/ganyu.png"
import splash from "./splash.png"

const g1 = "普通攻击·流天射术"
const g2 = "山泽麟迹"
const g3 = "降众天华"

export default {
    name: "Ganyu",
    chs: "甘雨",
    element: "Cryo",
    weapon: "Bow",
    star: 5,
    card,
    avatar,
    splash,
    skillMap: [
        { index: 0, chs: "一段伤害", group: g1 },
        { index: 1, chs: "二段伤害", group: g1 },
        { index: 2, chs: "三段伤害", group: g1 },
        { index: 3, chs: "四段伤害", group: g1 },
        { index: 4, chs: "五段伤害", group: g1 },
        { index: 5, chs: "六段伤害", group: g1 },
        { index: 6, chs: "瞄准射击", group: g1 },
        { index: 7, chs: "一段蓄力瞄准射击", group: g1 },
        { index: 8, chs: "霜华矢命中伤害", group: g1 },
        { index: 9, chs: "霜华矢·霜华绽发伤害", group: g1 },
        { index: 10, chs: "下坠期间伤害", group: g1 },
        { index: 11, chs: "低空坠地冲击伤害", group: g1 },
        { index: 12, chs: "高空坠地冲击伤害", group: g1 },
        { index: 13, chs: "技能伤害", group: g2 },
        { index: 14, chs: "冰棱伤害", group: g3 }
    ]
}