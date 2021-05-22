import { tableRock } from "./zhongli_utils";
import { getAttribute } from "@util/attribute";
import { charactersData } from "@asset/characters";


let skill = charactersData["zhongli"].skill;

let rowsE = [
    { key: "dmg1", chs: "岩脊伤害" },
    { key: "dmg2", chs: "共鸣伤害" },
    { key: "dmg2", chs: "长按伤害" },
];

export default function (artifacts, configObject, enemy) {
    let c = configObject.character;
    let w = configObject.weapon;
    let attribute = getAttribute(artifacts, c, w, configObject.buffs);

    let hasTalent2 = (c.level == 60 && c.ascend) || c.level > 60;
    let extraDmg = 0;
    if (hasTalent2) {
        extraDmg = attribute.life() * 0.019;
    }

    let e = tableRock(attribute, configObject, enemy, rowsE, "e", extraDmg);
    
    let idx = c.skill2 - 1;
    let shield = skill.e.shieldBase[idx] + skill.e.shieldExtra[idx] * attribute.life();

    return {
        e, shield
    }
}