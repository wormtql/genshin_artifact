import card from "./card.jpg"
import avatar from "@asset/badges/shanhugongxinhai.png"
import splash from "./splash.png"

const g1 = "普通攻击•水有常形"
const g2 = "海月之誓"
const g3 = "海人化羽"

export default {
    name: "SangonomiyaKokomi",
    chs: "珊瑚宫心海",
    element: "Hydro",
    weapon: "Catalyst",
    star: 5,
    card,
    avatar,
    splash,
    skillMap: [
        { index: 0, chs: "一段伤害", group: g1 },
        { index: 0, chs: "二段伤害", group: g1 },
        { index: 0, chs: "三段伤害", group: g1 },
        { index: 0, chs: "重击伤害", group: g1 },
        { index: 0, chs: "下坠期间伤害", group: g1 },
        { index: 0, chs: "低空坠地冲击伤害", group: g1 },
        { index: 0, chs: "高空坠地冲击伤害", group: g1 },
        { index: 0, chs: "治疗量", group: g2 },
        { index: 0, chs: "波纹伤害", group: g2 },
        { index: 0, chs: "技能伤害", group: g3 },
        { index: 0, chs: "命中治疗量", group: g3 },
    ]
}
