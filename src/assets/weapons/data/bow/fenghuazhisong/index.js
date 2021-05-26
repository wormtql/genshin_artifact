import tn from "./tn.png";
import { commonConfigRate } from "../../../common/utils";

export default {
    name: "fenghuazhisong",
    chs: "风花之颂",
    url: tn,
    star: 4,
    type: "bow",
    config: () => commonConfigRate("风花之颂"),
    effect: "风花之愿：施放元素战技时，获得风之花的悠古愿望加持，攻击力提升16%/20%/24%/28%/32%，持续6秒"
}