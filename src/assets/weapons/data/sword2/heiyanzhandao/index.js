import { commonConfigLevel } from "../../../common/utils";
import tn from "./tn.png";

export default {
    name: "heiyanzhandao",
    chs: "黑岩斩刀",
    url: tn,
    star: 4,
    type: "sword2",
    config: () => commonConfigLevel("黑岩斩刀", 3),
    effect: "乘胜追击：击败敌人后,攻击力提升12%/15%/18%/21%/24%,持续30秒。该效果至多叠加3层，每层持续时间独立。"
}