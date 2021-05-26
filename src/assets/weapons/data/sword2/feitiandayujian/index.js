import { commonConfigLevel } from "../../../common/utils";
import tn from "./tn.png";

export default {
    name: "feitiandayujian",
    chs: "飞天大御剑",
    url: tn,
    star: 3,
    type: "sword2",
    config: () => commonConfigLevel("飞天大御剑", 4),
    effect: "勇气：普通攻击和重击命中时，攻击力提高6%/7%/8%/9%/10%，持续6秒，最多叠加4层，该效果每0.5秒只能触发一次。",
}