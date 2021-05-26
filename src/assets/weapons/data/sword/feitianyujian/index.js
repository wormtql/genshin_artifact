import tn from "./tn.png";
import { commonConfigRate } from "../../../common/utils";

export default {
    name: "feitianyujian",
    chs: "飞天御剑",
    url: tn,
    star: 3,
    type: "sword",
    config: () => commonConfigRate("飞天御剑"),
    effect: "决心：施放元素爆发后，提高12%/15%/18%/21%/24%攻击力和移动速度，持续15秒。",
}