import { commonConfigLevel } from "../../../common/utils";
import tn from "./tn.png";

export default {
    name: "amosizhigong",
    chs: "阿莫斯之弓",
    url: tn,
    star: 5,
    type: "bow",
    config: () => commonConfigLevel("阿莫斯之弓", 5),
    effect: "矢志不忘：普通攻击和重击造成的伤害提升12%/15%/18%/21%/24%;箭矢发射后每经过0.1秒，伤害还会提升8%/10%/12%/14%/16%。至多提升5次。"
}