import { commonConfigLevel } from "../../../common/utils";
import tn from "./tn.png";

export default {
    name: "heiyanfeiyu",
    chs: "黑岩绯玉",
    url: tn,
    star: 4,
    type: "book",
    config: () => commonConfigLevel("黑岩绯玉", 3),
    effect: "乘胜追击：击败敌人后，攻击力提升12%/15%/18%/21%/24%，续30秒。该效果至多叠加3层，每层持续时间独立。"
}