import { commonConfigRate } from "../../../common/utils";
import tn from "./tn.png";

export default {
    name: "yucai",
    chs: "雨裁",
    url: tn,
    star: 4,
    type: "sword2",
    config: () => commonConfigRate("雨裁"),
    effect: "止水息雷：对处于水元素或雷元素影响下的敌人，造成的伤害提高20％/24%/28%/32%/36%。",
}