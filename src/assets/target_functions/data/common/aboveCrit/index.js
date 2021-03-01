import badge from "../badges/sword.png";
import config from "./config";

function halfCrit(config) {
    let threshold = config.tArgs.threshold;

    return function (attribute) {
        if (attribute.critical < threshold) {
            return attribute.critical;
        }
    
        return attribute.attack() * (1 + threshold * attribute.criticalDamage);
    };
}

export default {
    name: "aboveCrit",
    chs: "名字真难取",
    description: [
        "暴击率高于${threshold}时，堆攻击和爆伤，否则堆暴击率",
    ],
    tags: [
        "输出",
    ],
    func: halfCrit,
    "for": "common",
    badge,
    config,
    needConfig: true,
}