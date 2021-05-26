import tn from "./tn.png";
import { commonConfigLevel } from "../../../common/utils";

export default {
    name: "shizuoxinglian",
    chs: "试作星镰",
    url: tn,
    star: 4,
    type: "stick",
    config: () => commonConfigLevel("试作星镰", 2),
    effect: "嗜魔：施放元素战技后，普通攻击和重击造成的伤害提高8%/10%/12%/14%/16%。该效果持续12秒，最多叠加2层。"
}