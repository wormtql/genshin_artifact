import { tableRock } from "./zhongli_utils";
import { getAttribute } from "@util/attribute";


let rowsQ = [
    { key: "dmg1", chs: "技能伤害" },
];

export default function (artifacts, configObject, enemy) {
    let c = configObject.character;
    let w = configObject.weapon;
    let attribute = getAttribute(artifacts, c, w, configObject.buffs);

    let hasTalent2 = (c.level == 60 && c.ascend) || c.level > 60;
    let extraDmg = 0;
    if (hasTalent2) {
        extraDmg = attribute.life() * 0.33;
    }

    let q = tableRock(attribute, configObject, enemy, rowsQ, "q", extraDmg);

    return {
        q
    };
}