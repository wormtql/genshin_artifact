import { commonConfigRate } from "../../../common/utils";
import tn from "./tn.png";

export default {
    name: "zhengshizhimingtong",
    chs: "证誓之明瞳",
    url: tn,
    star: 4,
    type: "book",
    config: () => commonConfigRate("被动应用比例"),
    effect: "施放元素战技后，元素充能效率提升24%/30%/36%/42%/48%，持续10秒"
}