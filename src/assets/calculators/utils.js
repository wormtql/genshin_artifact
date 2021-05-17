import { getAttribute } from "@util/attribute";
import { charactersData } from "@asset/characters";
import reaction from "@/elemental_reaction/reaction_bonus";
import mergeArray from "@util/mergeArray";


function getBonus(attribute, element, skill) {
    return attribute[element + "Bonus"] + attribute[skill + "Bonus"] + attribute["bonus"];
}

export function damageNormal(attribute, cLevel, r, enemy, element, skill) {
    let atk = attribute.attack();
    let defensiveRatio = enemy.getDR(cLevel, attribute.defDown ?? 0);

    let resRatio = enemy.getRR(element);
    let damageBonus = 1 + getBonus(attribute, element, skill);

    let base = atk * r * defensiveRatio * resRatio * damageBonus;

    let crit = skill == "a" ? attribute["critical"] : attribute[skill + "Critical"];
    crit = Math.min(crit, 1);
    let cd = attribute.criticalDamage;

    return {
        crit: base * (1 + cd),
        nonCrit: base,
        expect: base * (1 + cd * crit),
    };
}

export function damageReaction(type, attribute, cLevel, r, enemy, element, skill) {
    let baseDmg = damageNormal(attribute, cLevel, r, enemy, element, skill);

    let amp = reaction.amp(attribute.elementalMastery);
    let rate;
    if (type === "melt") {
        amp += attribute.meltEnhance;
        rate = element === "fire" ? 2 : 1.5;
    } else if (type === "vaporize") {
        amp += attribute.vaporizeEnhance;
        rate = element === "water" ? 2 : 1.5;
    }

    return {
        crit: baseDmg.crit * rate * (1 + amp),
        nonCrit: baseDmg.nonCrit * rate * (1 + amp),
        expect: baseDmg.expect * rate * (1 + amp),
    };
}

export function tableNormalA(attribute, configObject, enemy, skillKeys, skillId) {
    let c = configObject.character;

    let cName = c.name;
    let skillData = charactersData[cName].skill;

    let skillLevel;
    if (skillId === "a" || skillId === "b" || skillId === "air") {
        skillLevel = c.skill1;
    } else if (skillId === "e") {
        skillLevel = c.skill2;
    } else {
        skillLevel = c.skill3;
    }

    let ret = [];
    for (let i = 0; i < skillKeys.length; i++) {
        let skillItem = skillKeys[i];
        ret.push(damageNormal(
            attribute,
            c.level,
            skillData[skillId][skillItem.key][skillLevel - 1],
            enemy,
            skillItem.element,
            skillItem.skill,
        ));
    }

    return ret;
}

export function tableNormal(artifacts, configObject, enemy, skillKeys, skillId) {
    let c = configObject.character;
    let w = configObject.weapon;
    let buffs = configObject.buffs;
    let attribute = getAttribute(artifacts, c, w, buffs);

    return tableNormalA(attribute, configObject, enemy, skillKeys, skillId);
}

export function tableReactionA(type, attribute, configObject, enemy, skillKeys, skillId) {
    let c = configObject.character;

    let cName = c.name;
    let skillData = charactersData[cName].skill;

    let skillLevel;
    if (skillId === "a" || skillId === "b" || skillId === "air") {
        skillLevel = c.skill1;
    } else if (skillId === "e") {
        skillLevel = c.skill2;
    } else {
        skillLevel = c.skill3;
    }

    let ret = [];
    for (let i = 0; i < skillKeys.length; i++) {
        let skillItem = skillKeys[i];
        ret.push(damageReaction(
            type,
            attribute,
            c.level,
            skillData[skillId][skillItem.key][skillLevel - 1],
            enemy,
            skillItem.element,
            skillItem.skill,
        ));
    }

    return ret;
}

export function tableReaction(type, artifacts, configObject, enemy, skillKeys, skillId) {
    let c = configObject.character;
    let w = configObject.weapon;
    let buffs = configObject.buffs;
    let attribute = getAttribute(artifacts, c, w, buffs);

    return tableReactionA(type, attribute, configObject, enemy, skillKeys, skillId);
}

// 创建[火元素伤害、蒸发、融化的]的函数
export function createFireFunction(skillKeys, skill) {
    return function (artifacts, configObject, enemy) {
        let c = configObject.character;
        let w = configObject.weapon;
        let attribute = getAttribute(artifacts, c, w, configObject.buffs);

        let ret = mergeArray(
            ["chs", skillKeys.map(item => item.chs)],
            ["fire", tableNormalA(attribute, configObject, enemy, skillKeys, skill)],
            ["fireMelt", tableReactionA("melt", attribute, configObject, enemy, skillKeys, skill)],
            ["fireVaporize", tableReactionA("vaporize", attribute, configObject, enemy, skillKeys, skill)],
        );

        return ret;
    }
}

export function createPhysicalFunction(skillKeys, skill) {
    return function (artifacts, configObject, enemy) {
        let c = configObject.character;
        let w = configObject.weapon;
        let attribute = getAttribute(artifacts, c, w, configObject.buffs);

        let ret =  mergeArray(
            ["chs", skillKeys.map(item => item.chs)],
            ["normal", tableNormalA(attribute, configObject, enemy, skillKeys, skill)],
        );
        
        return ret;
    }
}