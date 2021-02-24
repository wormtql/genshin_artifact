import tn from "./tn.png";

export default {
    name: "qianyangujian",
    chs: "千岩古剑",
    url: tn,
    star: 4,
    type: "sword2",
    args: [
        {
            name: "liyueCount",
            chs: "队伍中的璃月角色数量",
            type: "int",
            min: 0,
            max: 4,
            default: 0,
        }
    ]
}