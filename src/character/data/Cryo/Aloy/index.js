import card from "./card.png"
import avatar from "@asset/badges/ailuoyi.png"
import splash from "./splash.png"

const g1 = "普通攻击·快速射击"
const g2 = "冰尘雪野"
const g3 = "曙光预言"

export default {
    name: "Aloy",
    chs: "埃洛伊",
    element: "Cryo",
    weapon: "Bow",
    star: 5,
    card,
    avatar,
    splash,
    skillMap: [
        { index: 0, chs: "一段伤害-1", group: g1 },
        { index: 1, chs: "一段伤害-2", group: g1 },
        { index: 2, chs: "二段伤害", group: g1 },
        { index: 3, chs: "三段伤害", group: g1 },
        { index: 4, chs: "四段伤害", group: g1 },
        { index: 5, chs: "瞄准射击", group: g1 },
        { index: 6, chs: "满蓄力瞄准射击", group: g1 },
        { index: 7, chs: "下坠期间伤害", group: g1 },
        { index: 8, chs: "低空坠地冲击伤害", group: g1 },
        { index: 9, chs: "高空坠地冲击伤害", group: g1 },
        { index: 10, chs: "冰尘弹伤害", group: g2 },
        { index: 11, chs: "冷冻炸弹伤害", group: g2 },
        { index: 12, chs: "技能伤害", group: g3 },
    ]
}
