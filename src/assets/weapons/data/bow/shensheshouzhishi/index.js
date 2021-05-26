import { commonConfigPassive } from "../../../common/utils";
import tn from "./tn.png";

export default {
    name: "shensheshouzhishi",
    chs: "神射手之誓",
    url: tn,
    star: 3,
    type: "bow",
    config: () => commonConfigPassive("神射手之誓", "是否命中要害", "critical"),
    effect: "精准：针对要害造成的伤害提升24%/30%/36%/42%/48%。",
}