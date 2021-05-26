import { commonConfigRate } from "../../../common/utils";
import tn from "./tn.png";

export default {
    name: "jiajibaojue",
    chs: "甲级宝珏",
    url: tn,
    star: 3,
    type: "book",
    config: () => commonConfigRate("甲级宝珏"),
    effect: "奔袭战术：击败敌人后的15秒内，移动速度和攻击力提升12%/14%/16%/18%/20%。",
}