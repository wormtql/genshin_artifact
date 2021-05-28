import { tableRock } from "../../../utils";
import { getAttribute } from "@util/attribute";
import { charactersData } from "@asset/characters";


let skill = charactersData["abeiduo"].skill;

let rowsE = [
    { key: "dmg1", chs: "技能伤害" },
];

export function damageDef(attribute, cLevel, r, enemy, element, skill) {
    let def = attribute.defend();
    let defensiveRatio = enemy.getDR(cLevel, attribute.defDown ?? 0);

    let resRatio = enemy.getRR(element);
    let damageBonus = 1 + attribute.rockBonus + attribute.eBonus + attribute.bonus;

    let base = def * r * defensiveRatio * resRatio * damageBonus;

    let crit = skill == "a" ? attribute["critical"] : attribute[skill + "Critical"];
    crit = Math.min(crit, 1);
    let cd = attribute.criticalDamage;

    return {
        crit: base * (1 + cd),
        nonCrit: base,
        expect: base * (1 + cd * crit),
    };
}

export default function (artifacts, configObject, enemy) {
    let c = configObject.character;
    let w = configObject.weapon;
    let attribute = getAttribute(artifacts, c, w, configObject.buffs, configObject.artifactsConfig);

    let e = tableRock(attribute, configObject, enemy, rowsE, "e");
    e.push({
        chs: "刹那之花伤害",
        rock: damageDef(attribute, c.level, skill.e.dmg2[c.skill2 - 1], enemy, "rock", "e"),
    });

    return e;
}