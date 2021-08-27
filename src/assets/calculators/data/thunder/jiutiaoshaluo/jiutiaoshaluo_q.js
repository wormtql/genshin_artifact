import { getAttribute } from "@util/attribute";
import { tableThunder } from "../../../utils";


let rowsQ = [
    { key: "dmg1", chs: "天狗咒雷·金刚坏" },
    { key: "dmg2", chs: "天狗咒雷·雷砾" },
];


export default function (artifacts, configObject, enemy) {
    let c = configObject.character;
    let w = configObject.weapon;
    let attribute = getAttribute(artifacts, c, w, configObject.buffs, configObject.artifactsConfig);

    let q = tableThunder(attribute, configObject, enemy, rowsQ, "q");

    return {
        q,
    }
}