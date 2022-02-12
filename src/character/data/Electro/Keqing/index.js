import card from "./card.jpg"
import avatar from "@asset/badges/keqing.png"
// import skill from "./keqing.skill.js";
import splash from "./splash.png"

const g1 = "普通攻击·云来剑法"
const g2 = "星斗归位"
const g3 = "天街巡游"

export default {
    name: "Keqing",
    chs: "刻晴",
    element: "Electro",
    weapon: "Sword",
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
        { index: 6, chs: "重击伤害-1", group: g1 },
        { index: 7, chs: "重击伤害-2", group: g1 },
        { index: 8, chs: "下坠期间伤害", group: g1 },
        { index: 9, chs: "低空坠地冲击伤害", group: g1 },
        { index: 10, chs: "高空坠地冲击伤害", group: g1 },
        { index: 11, chs: "雷楔伤害", group: g2 },
        { index: 12, chs: "斩击伤害", group: g2 },
        { index: 13, chs: "雷暴连斩伤害/2", group: g2 },
        { index: 15, chs: "技能伤害", group: g3 },
        { index: 16, chs: "连斩伤害/8", group: g3 },
        { index: 18, chs: "最后一击伤害", group: g3 }
    ]
    // skill,
}