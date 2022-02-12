import card from "./card.jpg"
import avatar from "@asset/badges/xiangling.png"
import splash from "./splash.png"

const g1 = "普通攻击·白案功夫"
const g2 = "锅巴出击"
const g3 = "旋火轮"

export default {
    name: "Xiangling",
    chs: "香菱",
    element: "Pyro",
    weapon: "Polearm",
    star: 4,
    card,
    avatar,
    splash,
    skillMap: [
        { index: 0, chs: "一段伤害", group: g1 },
        { index: 1, chs: "二段伤害", group: g1 },
        { index: 2, chs: "三段伤害-1", group: g1 },
        { index: 3, chs: "三段伤害-2", group: g1 },
        { index: 5, chs: "四段伤害", group: g1 },
        { index: 7, chs: "五段伤害", group: g1 },
        { index: 8, chs: "重击伤害", group: g1 },
        { index: 9, chs: "下坠期间伤害", group: g1 },
        { index: 10, chs: "低空坠地冲击伤害", group: g1 },
        { index: 11, chs: "高空坠地冲击伤害", group: g1 },
        { index: 12, chs: "喷火伤害", group: g2 },
        { index: 13, chs: "一段挥舞伤害", group: g3 },
        { index: 14, chs: "二段挥舞伤害", group: g3 },
        { index: 15, chs: "三段挥舞伤害", group: g3 },
        { index: 16, chs: "旋火轮伤害", group: g3 },
    ]
}
