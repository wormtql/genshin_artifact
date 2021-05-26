import tn from "./tn.png";
import { commonConfigRate } from "../../../common/utils";

export default {
    name: "antiejian",
    chs: "暗铁剑",
    url: tn,
    star: 3,
    type: "sword",
    config: () => commonConfigRate("暗铁剑"),
    effect: "过载：触发超载、超导、感电或雷元素扩散反应后的12秒内，攻击力提高20%/25%/30%/35%/40%。",
}