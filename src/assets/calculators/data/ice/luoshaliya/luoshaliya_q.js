import { tableIce } from "../../../utils";
import { getAttribute } from "@util/attribute";


let rowsQ = [
    { key: "dmg11", chs: "技能伤害-1" },
    { key: "dmg12", chs: "技能伤害-2" },
    { key: "dmg2", chs: "冰枪持续伤害" },
];

export default function (artifacts, configObject, enemy) {
    let c = configObject.character;
    let w = configObject.weapon;
    let attribute = getAttribute(artifacts, c, w, configObject.buffs);

    let q = tableIce(attribute, configObject, enemy, rowsQ, "q");

    return {
        q,
    };
}