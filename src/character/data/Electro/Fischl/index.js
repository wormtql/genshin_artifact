import card from "./card.jpg"
import avatar from "@asset/badges/feixieer.png"
import splash from "./splash.png"

const g1 = "普通攻击·罪灭之矢"
const g2 = "夜巡影翼"
const g3 = "至夜幻现"

export default {
    name: "Fischl",
    chs: "菲谢尔",
    element: "Electro",
    weapon: "Bow",
    star: 4,
    card,
    avatar,
    skillMap: [
        { index: 0, chs: "一段伤害", group: g1 },
        { index: 1, chs: "二段伤害", group: g1 },
        { index: 2, chs: "三段伤害", group: g1 },
        { index: 3, chs: "四段伤害", group: g1 },
        { index: 4, chs: "五段伤害", group: g1 },
        { index: 5, chs: "瞄准射击", group: g1 },
        { index: 6, chs: "满蓄力瞄准射击", group: g1 },
        { index: 7, chs: "下坠期间伤害", group: g1 },
        { index: 8, chs: "低空坠地冲击伤害", group: g1 },
        { index: 9, chs: "高空坠地冲击伤害", group: g1 },
        { index: 10, chs: "奥兹攻击伤害", group: g2 },
        { index: 11, chs: "召唤伤害", group: g2 },
        { index: 12, chs: "落雷伤害", group: g3 },
    ],
    splash
    // skill,
}
