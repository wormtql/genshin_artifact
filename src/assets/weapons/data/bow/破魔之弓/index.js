// import tn from "./tn.png";
import { commonConfigRate } from "../../../common/utils";

export default {
    name: "pomozhigong",
    chs: "破魔之弓",
    // url: tn,
    url: "https://dummyimage.com/256x256/ffffff/000000&text=破魔之弓",
    star: 4,
    type: "bow",
    config: () => commonConfigRate("破魔之弓"),
    effect: "普通攻击造成的伤害提升16/20/24/28/32%，重击造成的伤害提升12/15/18/21/24%。当装备该武器的角色元素能量等于100%时，这个效果提升100%。"
}