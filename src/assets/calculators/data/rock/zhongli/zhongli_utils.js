import { charactersData } from "@asset/characters";
import mergeArray from "@util/mergeArray";

function getBonus(attribute, element, skill) {
    return attribute[element + "Bonus"] + attribute[skill + "Bonus"] + attribute["bonus"];
}

function damageNormal(attribute, cLevel, r, enemy, element, skill, extraDmg) {
    let atk = attribute.attack();
    let defensiveRatio = enemy.getDR(cLevel, attribute.defDown ?? 0);

    let resRatio = enemy.getRR(element);
    let damageBonus = 1 + getBonus(attribute, element, skill);

    let base = (atk * r + extraDmg) * defensiveRatio * resRatio * damageBonus;

    let crit = skill == "a" ? attribute["critical"] : attribute[skill + "Critical"];
    crit = Math.min(crit, 1);
    let cd = attribute.criticalDamage;

    return {
        crit: base * (1 + cd),
        nonCrit: base,
        expect: base * (1 + cd * crit),
    };
}

export function colNormal(attribute, configObject, enemy, rowConfigs, skillName, element, extraDamage) {
    let c = configObject.character;

    let cName = c.name;
    let skillData = charactersData[cName].skill;

    let skillLevel;
    let skill;
    if (skillName === "a" || skillName === "b" || skillName === "air") {
        skillLevel = c.skill1;
        skill = "a";
    } else if (skillName === "e") {
        skillLevel = c.skill2;
        skill = "e";
    } else {
        skillLevel = c.skill3;
        skill = "q";
    }

    let ret = [];
    for (let i = 0; i < rowConfigs.length; i++) {
        let skillItem = rowConfigs[i];
        ret.push(damageNormal(
            attribute,
            c.level,
            skillData[skill][skillItem.key][skillLevel - 1],
            enemy,
            element,
            skillName,
            extraDamage
        ));
    }

    return ret;
}

export function tableRock(attribute, configObject, enemy, rowConfigs, skillName, extraDamage) {
    return mergeArray(
        ["chs", rowConfigs.map(item => item.chs)],
        ["rock", colNormal(attribute, configObject, enemy, rowConfigs, skillName, "rock", extraDamage)],
    )
}

export function tablePhysical(attribute, configObject, enemy, rowConfigs, skillName, extraDamage) {
    let ret =  mergeArray(
        ["chs", rowConfigs.map(item => item.chs)],
        ["normal", colNormal(attribute, configObject, enemy, rowConfigs, skillName, "physical", extraDamage)],
    );
    return ret;
}