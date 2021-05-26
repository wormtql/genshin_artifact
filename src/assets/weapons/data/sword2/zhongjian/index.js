import { commonConfigRate } from "../../../common/utils";
import tn from "./tn.png";

export default {
    name: "zhongjian",
    chs: "钟剑",
    url: tn,
    star: 4,
    type: "sword2",
    config: () => commonConfigRate("钟剑"),
    effect: "叛逆的守护者：受到伤害时，生成一个伤害吸收等同于生命上限20%/23%/26%/29%/32%的护盾，持续10秒或者直到护盾失效，每45秒只能触发一次。角色处于护盾庇护下时，造成的伤害提升12%/15%/18%/21%/24%。"
}