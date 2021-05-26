import { commonConfigLevel } from "../../../common/utils";
import tn from "./tn.png";

export default {
    name: "tiefengci",
    chs: "铁蜂刺",
    url: tn,
    star: 4,
    type: "sword",
    config: () => commonConfigLevel("铁蜂刺", 2),
    effect: "注能之刺：造成元素伤害后的6秒内，角色造成的伤害提高6%/7.5%/9%/10.5%/12%，该效果最多叠加2层。该效果每1秒可以触发一次。",
}