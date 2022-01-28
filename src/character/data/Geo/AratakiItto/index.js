import card from "./card.jpg"
import avatar from "@asset/badges/huanglongyidou.png"
// import skill from "./huanglongyidou.skill.js";
import splash from "./splash.png"

const g1 = "普通攻击•喧哗屋传说"
const g2 = "魔杀绝技•赤牛发破！"
const g3 = "最恶鬼王•一斗轰临！！"

export default {
    name: "AratakiItto",
    chs: "荒泷一斗",
    element: "Geo",
    weapon: "Claymore",
    star: 5,
    card,
    avatar,
    splash,
    // skill,
    skillMap: [
        { index: 0, chs: "一段伤害", group: g1 },
        { index: 1, chs: "二段伤害", group: g1 },
        { index: 2, chs: "三段伤害", group: g1 },
        { index: 3, chs: "四段伤害", group: g1 },
        { index: 4, chs: "荒泷逆袈裟连斩伤害", group: g1 },
        { index: 5, chs: "荒泷逆袈裟终结伤害", group: g1 },
        { index: 6, chs: "左一文字斩伤害", group: g1 },
        { index: 7, chs: "下坠期间伤害", group: g1 },
        { index: 8, chs: "低空坠地冲击伤害", group: g1 },
        { index: 9, chs: "高空坠地冲击伤害", group: g1 },
        { index: 10, chs: "技能伤害", group: g2 }
    ]
}