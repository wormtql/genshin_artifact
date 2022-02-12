import card from "./card.jpg"
import avatar from "@asset/badges/dadaliya.png"
import splash from "./splash.png"

const g1 = "普通攻击·断雨"
const g2 = "魔王武装·狂澜"
const g3 = "极恶技·尽灭闪"

export default {
    name: "Tartaglia",
    chs: "达达利亚",
    element: "Hydro",
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
        { index: 7, chs: "满蓄力瞄准射击", group: g1 },
        { index: 8, chs: "断流·闪伤害", group: g1 },
        { index: 9, chs: "断流·破伤害", group: g1 },
        { index: 10, chs: "下坠期间伤害", group: g1 },
        { index: 11, chs: "低空坠地冲击伤害", group: g1 },
        { index: 12, chs: "高空坠地冲击伤害", group: g1 },
        { index: 13, chs: "状态爆发伤害", group: g2 },
        { index: 14, chs: "一段伤害", group: g2 },
        { index: 15, chs: "二段伤害", group: g2 },
        { index: 16, chs: "三段伤害", group: g2 },
        { index: 17, chs: "四段伤害", group: g2 },
        { index: 18, chs: "五段伤害", group: g2 },
        { index: 19, chs: "六段伤害-1", group: g2 },
        { index: 20, chs: "六段伤害-2", group: g2 },
        { index: 21, chs: "重击伤害-1", group: g2 },
        { index: 22, chs: "重击伤害-2", group: g2 },
        { index: 23, chs: "断流·斩伤害", group: g2 },
        { index: 24, chs: "技能伤害·近战", group: g3 },
        { index: 25, chs: "技能伤害·远程", group: g3 },
        { index: 26, chs: "断流·爆伤害", group: g3 },
    ]
}
