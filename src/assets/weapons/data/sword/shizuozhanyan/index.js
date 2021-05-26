import tn from "./tn.png";
import { commonConfigLevel } from "../../../common/utils";

export default {
    name: "shizuozhanyan",
    chs: "试作斩岩",
    url: tn,
    star: 4,
    type: "sword",
    config: () => commonConfigLevel("试做斩岩", 4),
    effect: "碎石：普通攻击或重击命中时，攻击力（原为基础攻击力）和防御力提高4%/5%/6%/7%/8%，持续6秒，最多叠加4层。该效果每0.3秒只能触发一次。",
}