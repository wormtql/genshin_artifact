import { tableRockCustom } from "../../../utils";
import { getAttribute } from "@util/attribute";
import { charactersData } from "@asset/characters";
import { capitalize } from "@util/utils";


let skill = charactersData["abeiduo"].skill;

let rowsE = [
    { key: "dmg1", chs: "技能伤害" },
];
let rowsE2 = [
    { chs: "刹那之花伤害" }
]

export function damageDef(attribute, cLevel, r, enemy, element, skill) {
    let def = attribute.defend();
    let defensiveRatio = enemy.getDR(cLevel, attribute.enemyDefDown ?? 0);

    const resAttributeKey = `enemy${capitalize(element)}Down`;
    let resRatio = enemy.getRR(element, attribute[resAttributeKey]);
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

    let skill2 = c.skill2;

    let dmg1Ratio = skill.e["dmg1"][skill2 - 1];
    let dmg2Ratio = skill.e["dmg2"][skill2 - 1];

    let e1 = tableRockCustom(attribute, configObject, enemy, rowsE, "e", dmg1Ratio * attribute.attack());
    let e2 = tableRockCustom(attribute, configObject, enemy, rowsE2, "e", dmg2Ratio * attribute.defend());

    let e = e1.concat(e2);

    return e;
}