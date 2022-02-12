import card from "./card.jpg"
import avatar from "@asset/badges/wendi.png"
import splash from "./splash.png"

const g1 = "普通攻击·神代射术"
const g2 = "高天之歌"
const g3 = "风神之诗"

export default {
    name: "Venti",
    chs: "温迪",
    element: "Anemo",
    weapon: "Bow",
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
        { index: 10, chs: "瞄准射击", group: g1 },
        { index: 11, chs: "满蓄力瞄准射击", group: g1 },
        { index: 12, chs: "下坠期间伤害", group: g1 },
        { index: 13, chs: "低空坠地冲击伤害", group: g1 },
        { index: 14, chs: "高空坠地冲击伤害", group: g1 },
        { index: 15, chs: "点按伤害", group: g2 },
        { index: 16, chs: "长按伤害", group: g2 },
        { index: 17, chs: "持续伤害", group: g3 },
        { index: 18, chs: "附加火元素伤害", group: g3 },
        { index: 19, chs: "附加雷元素伤害", group: g3 },
        { index: 20, chs: "附加水元素伤害", group: g3 },
        { index: 21, chs: "附加冰元素伤害", group: g3 },
    ]
}
