import tn from "./tn.png";
import { commonConfigLevel } from "../../../common/utils";

export default {
    name: "anxianglieshou",
    chs: "暗巷猎手",
    url: tn,
    star: 4,
    type: "bow",
    config: () => commonConfigLevel("暗巷猎手", 10),
    effect: "街巷伏击：装备该武器的角色处于队伍后台时，每1秒角色造成的伤害提升2%/2.5%/3%/3.5%/4%,最多通过这种方式获得20%/25%/30%/35%/40%的伤害提升；在场上超过4秒后，上述伤害提升效果每1秒会流失4%/5%/6%/7%/8%，直到降低至0%。"
}