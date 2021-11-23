import cup from "./cup.png";
import feather from "./feather.png";
import flower from "./flower.png";
import head from "./head.png";
import sand from "./sand.png";

export default {
    flower: {
        chs: "海染之花",
        url: flower,
    },
    feather: {
        chs: "渊宫之羽",
        url: feather,
    },
    sand: {
        chs: "离别之贝",
        url: sand,
    },
    cup: {
        chs: "真珠之笼",
        url: cup,
    },
    head: {
        chs: "海祇之冠",
        url: head,
    },
    chs: "海染砗磲",
    eng: "oceanHuedClam",
    minStar: 4,
    maxStar: 5,
    effectText: {
        chs: { 2: "治疗加成提高15%。", 4: "装备此圣遗物套装的角色对队伍中的角色进行治疗时，将产生持续3秒的海染泡沫，记录治疗的生命值回复量（包括溢出值）。持续时间结束时，海染泡沫将会爆炸，对周围的敌人造成90%累计回复量的伤害（该伤害结算方式同感电、超导等元素反应，但不受元素精通、等级或反应伤害加成效果影响）。每3.5秒至多产生一个海染泡沫；海染泡沫至多记录30000点回复量，含溢出部分的治疗量；自己的队伍中同时至多存在一个海染泡沫。装备此圣遗物套装的角色处于队伍后台时，依然能触发该效果。"}
    }
}