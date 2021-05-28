import { tableRock } from "../../../utils";
import { getAttribute } from "@util/attribute";
import { charactersData } from "@asset/characters";


let skill = charactersData["nuoaier"].skill;

let rowsQ = [
    { key: "dmg1", chs: "爆发伤害" },
    { key: "dmg2", chs: "技能伤害" },
];

export default function (artifacts, configObject, enemy) {
    let c = configObject.character;
    let w = configObject.weapon;
    let attribute = getAttribute(artifacts, c, w, configObject.buffs, configObject.artifactsConfig);

    let q = tableRock(attribute, configObject, enemy, rowsQ, "q");

    let atkLift = skill.q.atkLift[c.skill3 - 1] * attribute.defend();
    if (c.constellation >= 6) {
        atkLift += 0.5 * attribute.defend();
    }

    return {
        q, atkLift,
    };
}