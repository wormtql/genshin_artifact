import { commonConfigPassive } from "../../../common/utils";
import tn from "./tn.png";

export default {
    name: "tieyingkuojian",
    chs: "铁影阔剑",
    url: tn,
    star: 3,
    type: "sword2",
    config: () => commonConfigPassive("铁影阔剑", "是否应用被动"),
    effect: "不屈：生命值低于70%/75%/80%/85%/90%时，重击不会轻易被打断，并提高30%/35%/40%/45%/50%重击伤害。",
}