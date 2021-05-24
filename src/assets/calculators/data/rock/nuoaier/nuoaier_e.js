import { damageDefNormal } from "../../../utils";
import { getAttribute } from "@util/attribute";
import { charactersData } from "@asset/characters";


let skill = charactersData["nuoaier"].skill;


export default function (artifacts, configObject, enemy) {
    let c = configObject.character;
    let w = configObject.weapon;
    let attribute = getAttribute(artifacts, c, w, configObject.buffs);

    const idx = c.skill2 - 1;

    let e = [];
    e.push({
        chs: "技能伤害",
        rock: damageDefNormal(attribute, c.level, skill.e.dmg1[idx], enemy, "rock", "e"),
    });

    let shield = attribute.defend() * skill.e.shield1[idx] + skill.e.shield1Static[idx];
    let cure = attribute.defend() * skill.e.cure1[idx] + skill.e.cure1Static[idx];
    let prob = skill.e.prob[idx];

    return {
        e, cure, shield, prob,
    }
}