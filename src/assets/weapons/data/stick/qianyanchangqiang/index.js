import tn from "./tn.png";

export default {
    name: "qianyanchangqiang",
    chs: "千岩长枪",
    url: tn,
    star: 4,
    type: "stick",
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