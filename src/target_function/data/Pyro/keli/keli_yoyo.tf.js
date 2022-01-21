// badge to display
import badge from "@asset/badges/keli.png";
//import config from "./keli_yoyo.tcfg.vue";

export default {
    name: "keli_yoyo",    // 唯一的标识
    chs: "可莉通用模板by遥酱",     // 显示的中文
    description: [        // 描述
        // add strings
        "可莉常规输出函数",
    ],
    tags: [               // 标签
        // add strings
        "输出",
    ],
    "for": "Klee",      // 如果是角色专属，就写角色名，公共的写common
    badge,                // 显示的图标
    //config,               // 配置目标函数的vue组件（可选）
}