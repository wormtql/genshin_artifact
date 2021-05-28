import { tableFire } from "../../../utils";
import { getAttribute } from "@util/attribute";

let skillKeys = [
    {
        key: "dmg1",
        chs: "一段挥舞伤害",
    },
    {
        key: "dmg2",
        chs: "二段挥舞伤害",
    },
    {
        key: "dmg3",
        chs: "三段挥舞伤害",
    },
    {
        key: "dmg4",
        chs: "旋火轮伤害",
    },
];

export default function (artifacts, configObject, enemy) {
    let c = configObject.character;
    let w = configObject.weapon;
    let attribute = getAttribute(artifacts, c, w, configObject.buffs, configObject.artifactsConfig);

    let fireBonus = c.constellation >= 6 ? 0.15 : 0;
    attribute.fireBonus += fireBonus;

    return tableFire(attribute, configObject, enemy, skillKeys, "q");
}