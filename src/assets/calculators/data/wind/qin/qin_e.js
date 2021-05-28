import { tableWind } from "../../../utils";
import { getAttribute } from "@util/attribute";


let rowsE = [
    {
        key: "dmg1",
        chs: "技能伤害",
    },
];

export default function (artifacts, configObject, enemy) {
    let c = configObject.character;
    let w = configObject.weapon;
    let attribute = getAttribute(artifacts, c, w, configObject.buffs, configObject.artifactsConfig);

    let e = tableWind(attribute, configObject, enemy, rowsE, "e");

    return e;
}