import card from "./card.jpg"
import avatar from "@asset/badges/zhongli.png"
import splash from "./splash.png"

const g1 = "普通攻击·岩雨"
const g2 = "地心"
const g3 = "天星"

export default {
    name: "Zhongli",
    chs: "钟离",
    element: "Geo",
    weapon: "Polearm",
    star: 5,
    card,
    avatar,
    splash,
    skillMap: [
        { index: 0, chs: "一段伤害", group: g1 },
        { index: 1, chs: "二段伤害", group: g1 },
        { index: 2, chs: "三段伤害", group: g1 },
        { index: 3, chs: "四段伤害", group: g1 },
        { index: 4, chs: "五段伤害/4", group: g1 },
        { index: 5, chs: "六段伤害", group: g1 },
        { index: 6, chs: "重击伤害", group: g1 },
        { index: 7, chs: "下坠期间伤害", group: g1 },
        { index: 8, chs: "低空坠地冲击伤害", group: g1 },
        { index: 9, chs: "高空坠地冲击伤害", group: g1 },
        { index: 10, chs: "岩脊伤害", group: g2 },
        { index: 11, chs: "共鸣伤害", group: g2 },
        { index: 12, chs: "长按伤害", group: g2 },
        { index: 13, chs: "技能伤害", group: g3 }
    ]
}
