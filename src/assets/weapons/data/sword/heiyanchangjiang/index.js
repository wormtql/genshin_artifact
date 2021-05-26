import tn from "./tn.png";
import { commonConfigLevel } from "../../../common/utils";

export default {
    name: "heiyanchangjian",
    chs: "黑岩长剑",
    url: tn,
    star: 4,
    type: "sword",
    config: () => commonConfigLevel("黑岩长剑", 3),
    effect: "乘胜追击：击败敌人后，攻击力提升12%/15%/18%/21%/24%，持续30秒。该效果至多叠加三层，每层持续时间独立。",
}