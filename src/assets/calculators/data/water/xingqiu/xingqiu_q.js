import { tableWater } from "../../../utils";
import { getAttribute } from "@util/attribute";


let rowsQ = [
    {
        key: "dmg1",
        chs: "剑雨伤害",
    },
];

export default function (artifacts, configObject, enemy) {
    let c = configObject.character;
    let w = configObject.weapon;
    let attribute = getAttribute(artifacts, c, w, configObject.buffs);

    let q = tableWater(attribute, configObject, enemy, rowsQ, "q");

    return {
        q
    }
}