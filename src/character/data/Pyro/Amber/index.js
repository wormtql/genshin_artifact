import card from "./card.jpg"
import splash from "./splash.png"
import avatar from "@asset/badges/anbo.png"

export default {
    card,
    splash,
    name: "Amber",
    chs: "安柏",
    element: "Pyro",
    weapon: "Bow",
    star: 4,
    avatar,
    skillMap: [
        { index: 0, chs: "普攻1段", group: "普通攻击" },
        { index: 1, chs: "普攻2段", group: "普通攻击" },
        { index: 2, chs: "普攻3段", group: "普通攻击" },
        { index: 3, chs: "普攻4段", group: "普通攻击" },
        { index: 4, chs: "普攻5段", group: "普通攻击" },
        { index: 5, chs: "瞄准射击", group: "重击" },
        { index: 6, chs: "满蓄力瞄准射击", group: "重击" },
        { index: 7, chs: "下坠期间伤害", group: "下落攻击" },
        { index: 8, chs: "低空坠地冲击伤害", group: "下落攻击" },
        { index: 9, chs: "高空坠地冲击伤害", group: "下落攻击" },
        { index: 10, chs: "爆炸伤害", group: "元素战技" },
        { index: 11, chs: "箭雨单次伤害", group: "元素爆发" },
        { index: 12, chs: "箭雨总伤害", group: "元素爆发" },
    ]
}