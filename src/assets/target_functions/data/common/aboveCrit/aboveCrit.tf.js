import badge from "@asset/badges/sword.png";
import config from "./AboveCrit.tcfg";

export default {
    name: "aboveCrit",
    chs: "固定暴击率",
    description: [
        "优先堆暴击率到给定阈值，再堆攻击和爆伤",
    ],
    tags: [
        "输出",
    ],
    "for": "common",
    badge,
    config,
}