import { commonConfigRate } from "../../../common/utils";
import tn from "./tn.png";

export default {
    name: "shizuodanyue",
    chs: "试作澹月",
    url: tn,
    star: 4,
    type: "bow",
    config: () => commonConfigRate("试作澹月"),
    effect: "离簇不归：瞄准射击时，若命中要害，则提升10%移动速度与36%/45%/54%/63%/72%攻击力，持续10秒。"
}