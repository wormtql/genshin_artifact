import { commonConfigLevel } from "../../../common/utils";
import tn from "./tn.png";

export default {
    name: "chigujian",
    chs: "螭骨剑",
    url: tn,
    star: 4,
    type: "sword2",
    config: () => commonConfigLevel("螭骨剑", 5),
    effect: "破浪：角色在场上时，每4秒提升6%/7%/8%/9%/10%造成的伤害，3%/2.7%/2.4%/2.2%/2%受到的伤害。该效果最多叠加5层，不随角色退场重置，受到伤害后会减少1层效果"
}