import tn from "./tn.png";
import { commonConfigLevel } from "../../../common/utils";

export default {
    name: "heiyanciqiang",
    chs: "黑岩刺枪",
    url: tn,
    star: 4,
    type: "stick",
    config: () => commonConfigLevel("黑岩刺枪", 3),
    effect: "乘胜追击：击败敌人后，攻击力提升12%/15%/18%/21%/24%，持续30秒。该效果至多叠加3层，每层持续时间独立。"
}