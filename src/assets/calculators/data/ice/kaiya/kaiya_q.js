import { tableIce } from "../../../utils";
import { getAttribute } from "@util/attribute";


let rowsQ = [
    { key: "dmg1", chs: "技能伤害" },
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