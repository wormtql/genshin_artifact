import tn from "./tn.png";
import config from "./TangongConfig.wcfg";

export default {
    name: "tangong",
    chs: "弹弓",
    url: tn,
    star: 3,
    type: "bow",
    config,
    effect: "弹弓：普通攻击和瞄准射击时，箭矢若在发射后的0.3秒内击中敌人，则造成的伤害增加36%/42%/48%/54%/60%；否则，造成的伤害下降10%。"
}