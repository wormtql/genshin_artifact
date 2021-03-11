import badge from "@asset/badges/fire_slime.png";
import config from "./CritExpectConfig";

export default {
    name: "critExpect",
    chs: "固定暴击率-期望伤害",
    description: [
        "优先堆暴击率到给定阈值，再使得期望伤害最高"
    ],
    tags: [
        "期望", "输出",
    ],
    "for": "common",
    badge,
    needConfig: true,
    config,
}