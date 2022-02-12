import card from "./card.jpg"
import avatar from "@asset/badges/xiao.png"
import splash from "./splash.png"

const g1 = "普通攻击·卷积微尘"
const g2 = "风轮两立"
const g3 = "靖妖傩舞"

export default {
    name: "Xiao",
    chs: "魈",
    element: "Anemo",
    weapon: "Polearm",
    star: 5,
    card,
    avatar,
    splash,
    skillMap: [
        { index: 0, chs: "一段伤害-1", group: g1 },
        { index: 1, chs: "一段伤害-2", group: g1 },
        { index: 3, chs: "二段伤害", group: g1 },
        { index: 4, chs: "三段伤害", group: g1 },
        { index: 5, chs: "四段伤害-1", group: g1 },
        { index: 6, chs: "四段伤害-2", group: g1 },
        { index: 8, chs: "五段伤害", group: g1 },
        { index: 9, chs: "六段伤害", group: g1 },
        { index: 10, chs: "重击伤害", group: g1 },
        { index: 11, chs: "下坠期间伤害", group: g1 },
        { index: 12, chs: "低空坠地冲击伤害", group: g1 },
        { index: 13, chs: "高空坠地冲击伤害", group: g1 },
        { index: 14, chs: "技能伤害", group: g2 },
    ]
}
