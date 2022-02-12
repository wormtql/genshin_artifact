import card from "./card.jpg"
import avatar from "@asset/badges/yunjin.png"
import splash from "./splash.png"

const g1 = "普通攻击·拂云出手"
const g2 = "旋云开相"
const g3 = "破嶂见旌仪"

export default {
    name: "Yunjin",
    chs: "云堇",
    element: "Geo",
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
        { index: 4, chs: "四段伤害-1", group: g1 },
        { index: 5, chs: "四段伤害-2", group: g1 },
        { index: 6, chs: "五段伤害", group: g1 },
        { index: 7, chs: "重击伤害", group: g1 },
        { index: 8, chs: "下坠期间伤害", group: g1 },
        { index: 9, chs: "低空坠地冲击伤害", group: g1 },
        { index: 10, chs: "高空坠地冲击伤害", group: g1 },
        { index: 11, chs: "点按伤害", group: g2 },
        { index: 12, chs: "一段蓄力伤害", group: g2 },
        { index: 13, chs: "二段蓄力伤害", group: g2 },
        { index: 14, chs: "技能伤害", group: g3 },
    ]
}
