import { tableIce } from "../../../utils";
import { getAttribute } from "@util/attribute";
import { charactersData } from "@asset/characters";


let skill = charactersData["diaona"].skill;

let rowsE = [
    { key: "dmg1", chs: "猫抓伤害（个）" },
];

export default function (artifacts, configObject, enemy) {
    let c = configObject.character;
    let w = configObject.weapon;
    let attribute = getAttribute(artifacts, c, w, configObject.buffs, configObject.artifactsConfig);

    if (c.constellation >= 2) {
        attribute.eBonus += 0.15;
    }

    let e = tableIce(attribute, configObject, enemy, rowsE, "e");
    
    const idx = c.skill2 - 1;
    let shield = skill.e.shield1[idx] * attribute.life() + skill.e.shield1Static[idx];

    return {
        e, shield,
    }
}