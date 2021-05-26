import { commonConfigLevel } from "../../../common/utils";
import tn from "./tn.png";

export default {
    name: "baiyingjian",
    chs: "白影剑",
    url: tn,
    star: 4,
    type: "sword2",
    config: () => commonConfigLevel("白影剑", 4),
    effect: "注能之锋：普通攻击和重击命中后，攻击力和防御力提高6%/7.5%/9%/10.5%/12%。该效果持续6秒，最多叠加4层，每0.5秒只能触发一次。",
}