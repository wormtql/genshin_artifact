import { commonConfigRate } from "../../../common/utils";
import tn from "./tn.png";

export default {
    name: "xialilongyin",
    chs: "匣里龙吟",
    url: tn,
    star: 4,
    type: "sword",
    config: () => commonConfigRate("匣里龙吟"),
    effect: "踏火息雷：对处于火元素或雷元素影响下的敌人，造成的伤害提高20%/24%/28%/32%/36%。",
}