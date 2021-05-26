import tn from "./tn.png";
import { commonConfigLevel } from "../../../common/utils";

export default {
    name: "zongshilieqiang",
    chs: "宗室猎枪",
    url: tn,
    star: 4,
    type: "stick",
    config: () => commonConfigLevel("宗室猎枪", 5),
    effect: "专注：攻击造成伤害时，暴击率提升8%/10%/12%/14%/16%,最多堆叠5次。攻击造成暴击后，移除已有的专注效果。"
}