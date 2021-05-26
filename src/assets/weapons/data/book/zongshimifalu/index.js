import { commonConfigLevel } from "../../../common/utils";
import tn from "./tn.png";

export default {
    name: "zongshimifalu",
    chs: "宗室秘法录",
    url: tn,
    star: 4,
    type: "book",
    config: () => commonConfigLevel("宗室秘法录", 5),
    effect: "专注：攻击造成伤害时，暴击率提升8%/10%/12%/14%/16%，最多堆叠5次。攻击造成暴击后，移除已有的专注效果。"
}