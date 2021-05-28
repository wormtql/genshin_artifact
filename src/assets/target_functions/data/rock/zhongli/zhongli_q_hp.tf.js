import badge from "@asset/badges/zhongli.png";
import config from "./ZhongliQHP.tcfg.vue";

export default {
    name: "zhongliQ_HP",
    chs: "钟离-天动万象-最低生命值",
    description: [
        "在生命值大于一定值的条件下，使得钟离Q技能的伤害期望最高",
        "若等级不足会忽略第二天赋",
    ],
    tags: [
        "钟离",
    ],
    "for": "zhongli",
    badge,
    config,
}