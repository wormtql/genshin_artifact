import { commonConfigLevel } from "../../../common/utils";
import tn from "./tn.png";

export default {
    name: "heiyanzhangong",
    chs: "黑岩战弓",
    url: tn,
    star: 4,
    type: "bow",
    config: () => commonConfigLevel("黑岩战弓", 3),
    effect: "乘胜追击：击败敌人后,攻击力提升12%/15%/18%/21%/24%,持续30秒。该效果至多叠加3层,每层持续时间独立。"
}