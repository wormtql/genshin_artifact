import { tableIce } from "../../../utils";
import { getAttribute } from "@util/attribute";
import { charactersData } from "@asset/characters";


let skill = charactersData["qiqi"].skill;

let rowsE = [
    { key: "dmg1", chs: "技能伤害" },
    { key: "dmg2", chs: "寒病鬼差伤害" },
];

export default function (artifacts, configObject, enemy) {
    let c = configObject.character;
    let w = configObject.weapon;
    let attribute = getAttribute(artifacts, c, w, configObject.buffs);

    let e = tableIce(attribute, configObject, enemy, rowsE, "e");
    
    const idx = c.skill2 - 1;
    let cure1 = skill.e.cure1[idx] * attribute.attack() + skill.e.cure1Static[idx];
    let cure2 = skill.e.cure2[idx] * attribute.attack() + skill.e.cure2Static[idx];

    return {
        e, cure1, cure2,
    }
}