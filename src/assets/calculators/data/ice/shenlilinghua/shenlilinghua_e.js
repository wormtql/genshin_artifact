import { tableIce } from "../../../utils";
import { getAttribute } from "@util/attribute";


let rowsE = [
    { key: "dmg1", chs: "技能伤害" },
];

export default function (artifacts, configObject, enemy, { afterTalent2 }) {
    let c = configObject.character;
    let w = configObject.weapon;
    let attribute = getAttribute(artifacts, c, w, configObject.buffs, configObject.artifactsConfig);

    if (afterTalent2) {
        attribute.iceBonus += 0.18;
    }

    let e = tableIce(attribute, configObject, enemy, rowsE, "e");

    return {
        e
    }
}