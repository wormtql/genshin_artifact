import { commonConfigRate } from "../../../common/utils";
import tn from "./tn.png";

export default {
    name: "langdemolu",
    chs: "狼的末路",
    url: tn,
    star: 5,
    type: "sword2",
    config: () => commonConfigRate("狼的末路"),
    effect: "如狼般狩猎者：攻击力提高20%/25%/30%/35%/40%；攻击命中生命值低于30%的敌人时，队伍中所有成员的攻击力提高40%/50%/60%/70%/80%，持续12秒。该效果30秒只能触发一次。"
}