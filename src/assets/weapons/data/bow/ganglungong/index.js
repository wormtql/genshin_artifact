import { commonConfigLevel } from "../../../common/utils";
import tn from "./tn.png";

export default {
    name: "ganglungong",
    chs: "钢轮弓",
    url: tn,
    star: 4,
    type: "bow",
    config: () => commonConfigLevel("钢轮弓", 4),
    effect: "注能之矢：普通攻击和瞄准攻击命中时，提升4%/5%/6%/7%/8%攻击力与1.2%/1.5%/1.8%/2.1%/2.4%普通攻击速度。该效果持续6秒，最多可以叠加4层，每0.3秒只能触发一次。"
}