import { commonConfigRate } from "../../../common/utils";
import tn from "./tn.png";

export default {
    name: "yayugong",
    chs: "鸦羽弓",
    url: tn,
    star: 3,
    type: "bow",
    config: () => commonConfigRate("鸦羽弓"),
    effect: "踏火止水：对处于火元素或水元素影响下的敌人，造成的伤害提高12%/15%/18%/21%/24%。"
}