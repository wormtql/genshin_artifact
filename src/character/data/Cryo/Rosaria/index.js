import card from "./card.png"
import avatar from "@asset/badges/luoshaliya.png"
import splash from "./splash.png"

const g1 = "普通攻击·教会枪术"
const g2 = "噬罪的告解"
const g3 = "终命的圣礼"

export default {
    name: "Rosaria",
    chs: "罗莎莉亚",
    element: "Cryo",
    weapon: "Polearm",
    star: 4,
    card,
    avatar,
    splash,
    skillMap: [
        { index: 0, chs: "一段伤害", group: g1 },
        { index: 1, chs: "二段伤害", group: g1 },
        { index: 2, chs: "三段伤害/2", group: g1 },
        { index: 3, chs: "四段伤害", group: g1 },
        { index: 4, chs: "五段伤害-1", group: g1 },
        { index: 5, chs: "五段伤害-2", group: g1 },
        { index: 6, chs: "重击伤害", group: g1 },
        { index: 7, chs: "下坠期间伤害", group: g1 },
        { index: 8, chs: "低空坠地冲击伤害", group: g1 },
        { index: 9, chs: "高空坠地冲击伤害", group: g1 },
        { index: 10, chs: "技能伤害-1", group: g2 },
        { index: 11, chs: "技能伤害-2", group: g2 },
        { index: 12, chs: "技能伤害-1", group: g3 },
        { index: 13, chs: "技能伤害-2", group: g3 },
        { index: 14, chs: "冰枪持续伤害", group: g3 },
    ]
}
