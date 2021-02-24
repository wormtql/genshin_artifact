import tn from "./tn.png";

export default {
    name: "humozhizhang",
    chs: "护摩之杖",
    url: tn,
    star: 5,
    type: "stick",
    args: [
        {
            name: "lifeBelow50",
            chs: "生命值低于50%",
            type: "bool",
            default: false,
        }
    ]
}