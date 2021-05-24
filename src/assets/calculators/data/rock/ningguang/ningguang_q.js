import { tableRock } from "../../../utils";
import { getAttribute } from "@util/attribute";


let rowsQ = [
    { key: "dmg1", chs: "宝石伤害（颗）" },
];

export default function (artifacts, configObject, enemy) {
    let c = configObject.character;
    let w = configObject.weapon;
    let attribute = getAttribute(artifacts, c, w, configObject.buffs);

    let q = tableRock(attribute, configObject, enemy, rowsQ, "q");

    return {
        q
    };
}