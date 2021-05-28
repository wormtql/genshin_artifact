import { tableIce } from "../../../utils";
import { getAttribute } from "@util/attribute";
import { charactersData } from "@asset/characters";


let skill = charactersData["ganyu"].skill;

let rowsE = [
    { key: "dmg1", chs: "技能伤害" },
];

export default function (artifacts, configObject, enemy) {
    let c = configObject.character;
    let w = configObject.weapon;
    let attribute = getAttribute(artifacts, c, w, configObject.buffs, configObject.artifactsConfig);

    let e = tableIce(attribute, configObject, enemy, rowsE, "e");
    
    const idx = c.skill2 - 1;
    let hp = attribute.life() * skill.e.hp[idx];

    return {
        e, hp,
    }
}