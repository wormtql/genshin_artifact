import badge from "../badges/thunder_slime.png";
import config from "./CustomConfig";


function f(config) {
    let code = config.tArgs.code;

    let func = new Function("attribute", code);

    return func;
}

export default {
    name: "code",
    chs: "代码（仅供娱乐）",
    description: [
        "使用代码自定义期望函数"
    ],
    tags: [
        "代码",
    ],
    func: f,
    "for": "common",
    badge,
    needConfig: true,
    config,
}