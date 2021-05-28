import { getAttribute } from "@util/attribute";
import { charactersData } from "@asset/characters";

let skill = charactersData["babala"].skill;

export default function (artifacts, configObject) {
    let c = configObject.character;
    let w = configObject.weapon;
    let attribute = getAttribute(artifacts, c, w, configObject.buffs, configObject.artifactsConfig);

    let idx = c.skill3 - 1;
    let cure = skill.q.cure1[idx] * attribute.life() + skill.q.cure1Static[idx];

    return {
        cure,
    }
}