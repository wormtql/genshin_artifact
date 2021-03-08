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
    chs: "固定暴击率",
    description: [
        "优先堆暴击率到给定阈值，再堆攻击和爆伤",
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