import card from "./card.png"
import avatar from "@asset/badges/leidianjiangjun.png"
import splash from "./splash.png"

const g1 = "普通攻击•源流"
const g2 = "神变•恶曜开眼"
const g3 = "奥义•梦想真说"

export default {
    name: "RaidenShogun",
    chs: "雷电将军",
    element: "Electro",
    weapon: "Polearm",
    star: 5,
    card,
    avatar,
    splash,
    skillMap: [
        { index: 0, chs: "一段伤害", group: g1 },
        { index: 1, chs: "二段伤害", group: g1 },
        { index: 2, chs: "三段伤害", group: g1 },
        { index: 3, chs: "四段伤害-1", group: g1 },
        { index: 4, chs: "四段伤害-2", group: g1 },
        { index: 5, chs: "五段伤害", group: g1 },
        { index: 6, chs: "重击伤害", group: g1 },
        { index: 7, chs: "下坠期间伤害", group: g1 },
        { index: 8, chs: "低空坠地冲击伤害", group: g1 },
        { index: 9, chs: "高空坠地冲击伤害", group: g1 },
        { index: 10, chs: "技能伤害", group: g2 },
        { index: 11, chs: "协同攻击伤害", group: g2 },
        { index: 12, chs: "梦想一刀基础伤害", group: g3 },
        { index: 13, chs: "一段伤害", group: g3 },
        { index: 14, chs: "二段伤害", group: g3 },
        { index: 15, chs: "三段伤害", group: g3 },
        { index: 16, chs: "四段伤害-1", group: g3 },
        { index: 17, chs: "四段伤害-2", group: g3 },
        { index: 18, chs: "五段伤害", group: g3 },
        { index: 19, chs: "重击伤害-1", group: g3 },
        { index: 20, chs: "重击伤害-2", group: g3 },
        { index: 21, chs: "下坠期间伤害", group: g3 },
        { index: 22, chs: "低空坠地冲击伤害", group: g3 },
        { index: 23, chs: "高空坠地冲击伤害", group: g3 },
    ]
}
