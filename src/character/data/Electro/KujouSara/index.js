import card from "./card.jpg"
import avatar from "@asset/badges/jiutiaoshaluo.png"
import splash from "./splash.png"

const g1 = "普通攻击•天狗传弓术"
const g2 = "鸦羽天狗霆雷召咒"
const g3 = "煌煌千道镇式"

export default {
    name: "KujouSara",
    chs: "九条裟罗",
    element: "Electro",
    weapon: "Bow",
    star: 4,
    card,
    avatar,
    splash,
    skillMap: [
        { index: 0, chs: "一段伤害", group: g1 },
        { index: 1, chs: "二段伤害", group: g1 },
        { index: 2, chs: "三段伤害", group: g1 },
        { index: 3, chs: "四段伤害", group: g1 },
        { index: 4, chs: "五段伤害", group: g1 },
        { index: 5, chs: "瞄准射击", group: g1 },
        { index: 6, chs: "满蓄力瞄准射击", group: g1 },
        { index: 7, chs: "下坠期间伤害", group: g1 },
        { index: 8, chs: "低空坠地冲击伤害", group: g1 },
        { index: 9, chs: "高空坠地冲击伤害", group: g1 },
        { index: 10, chs: "天狗咒雷•伏伤害", group: g2 },
        { index: 11, chs: "天狗咒雷•金刚坏伤害", group: g3 },
        { index: 12, chs: "天狗咒雷•雷砾伤害", group: g3 }
    ]
}