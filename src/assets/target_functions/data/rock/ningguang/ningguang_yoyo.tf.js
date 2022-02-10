// badge to display
import badge from "@asset/badges/ningguang.png";
import config from "./ningguang_yoyo.tcfg.vue";

export default {
    name: "ningguang_yoyo",    // 唯一的标识
    chs: "凝光通用模板by遥酱",     // 显示的中文
    description: [        // 描述
        // add strings
        "凝光常规输出函数，可选择是否速切",
    ],
    tags: [               // 标签
        // add strings
        "输出",
    ],
    "for": "ningguang",      // 如果是角色专属，就写角色名，公共的写common
    badge,                // 显示的图标
    config,               // 配置目标函数的vue组件（可选）
}