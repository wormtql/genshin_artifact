import { tableIce } from "../../../utils";
import { getAttribute } from "@util/attribute";
import { charactersData } from "@asset/characters";


let skill = charactersData["diaona"].skill;

let rowsQ = [
    { key: "dmg1", chs: "技能伤害" },
    { key: "dmg2", chs: "领域持续伤害" },
];

export default function (artifacts, configObject, enemy) {
    let c = configObject.character;
    let w = configObject.weapon;
    let attribute = getAttribute(artifacts, c, w, configObject.buffs);

    let q = tableIce(attribute, configObject, enemy, rowsQ, "q");
    const idx = c.skill3 - 1;
    let cure = skill.q.cure1[idx] * attribute.life() + skill.q.cure1Static[idx];

    return {
        q, cure,
    };
}