import { tableThunder } from "../../../utils";
import { getAttribute } from "@util/attribute";


let rowsQ = [
    {
        key: "dmg1",
        chs: "落雷伤害",
    },
];

export default function (artifacts, configObject, enemy) {
    let c = configObject.character;
    let w = configObject.weapon;
    let attribute = getAttribute(artifacts, c, w, configObject.buffs);

    return tableThunder(attribute, configObject, enemy, rowsQ, "q");
}