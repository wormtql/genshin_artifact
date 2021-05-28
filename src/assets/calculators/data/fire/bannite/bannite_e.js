import { tableFire } from "../../../utils";
import { getAttribute } from "@util/attribute";

let skillKeys = [
    {
        key: "dmg1",
        chs: "点按伤害",
    },
    {
        key: "dmg21",
        chs: "一段蓄力-1",
    },
    {
        key: "dmg22",
        chs: "一段蓄力-2",
    },
    {
        key: "dmg31",
        chs: "二段蓄力-1",
    },
    {
        key: "dmg32",
        chs: "二段蓄力-2",
    },
    {
        key: "dmg4",
        chs: "爆炸伤害",
    },
];

export default function (artifacts, configObject, enemy) {
    let c = configObject.character;
    let w = configObject.weapon;
    let attribute = getAttribute(artifacts, c, w, configObject.buffs, configObject.artifactsConfig);

    return tableFire(attribute, configObject, enemy, skillKeys, "e");
}