import tn from "./tn.png";
import { commonConfigRate } from "../../../common/utils";

export default {
    name: "lengren",
    chs: "冷刃",
    url: tn,
    star: 3,
    type: "sword",
    config: () => commonConfigRate("冷刃"),
    effect: "止水融冰：对处于水元素或冰元素影响下的敌人，造成的伤害提高12%/15%/18%/21%/24%。",
}