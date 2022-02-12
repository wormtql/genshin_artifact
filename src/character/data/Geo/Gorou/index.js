import card from "./card.jpg"
import avatar from "@asset/badges/wulang.png"
import splash from "./splash.png"

const g1 = "普通攻击•呲牙裂扇箭"
const g2 = "犬坂吠吠方圆阵"
const g3 = "兽牙逐突形胜战法"

export default {
    name: "Gorou",
    chs: "五郎",
    element: "Geo",
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
        { index: 4, chs: "瞄准射击", group: g1 },
        { index: 5, chs: "满蓄力瞄准射击", group: g1 },
        { index: 6, chs: "下坠期间伤害", group: g1 },
        { index: 7, chs: "低空坠地冲击伤害", group: g1 },
        { index: 8, chs: "高空坠地冲击伤害", group: g1 },
        { index: 9, chs: "技能伤害", group: g2 },
        { index: 10, chs: "技能伤害", group: g3 },
        { index: 11, chs: "岩晶崩破伤害", group: g3 }
    ]
}