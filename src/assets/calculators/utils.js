// import { getAttribute } from "@util/attribute";
import { charactersData } from "@asset/characters";
import reaction from "@/elemental_reaction/reaction_bonus";
import mergeArray from "@util/mergeArray";
import { capitalize } from "@util/utils";


// get bonus from element and skill
function getBonus(attribute, element, skill) {
    return attribute[element + "Bonus"] + attribute[skill + "Bonus"] + attribute["bonus"];
}


// get damage, given character level(for defense ratio), enemy, baseDmg
// note that baseDmg will be plused by attribute base bonus
// for example, weapon 不灭月华 will add base attack by hp percentage
// dmg: extra base damage, base damage bonused by weapon is calculated from attribude
export function damageDelegate(attribute, cLevel, dmg, enemy, element, skill, extraBonus = []) {
    const defensiveRatio = enemy.getDR(cLevel, attribute.enemyDefDown ?? 0);

    const resDownAttributeName = `enemy${capitalize(element)}Down`;
    const resDown = attribute[resDownAttributeName] ?? 0;
    const resRatio = enemy.getRR(element, resDown);
    
    const damageBonus = 1 + getBonus(attribute, element, skill) + extraBonus.reduce((a, b) => a + b, 0);

    let baseDmg = dmg;
    if (skill === "a") {
        baseDmg += attribute.getNormalAttackBaseDamage(0);
    } else if (skill === "b") {
        baseDmg += attribute.getChargedAttackBaseDamage(0);
    } else if (skill === "air") {
        baseDmg += attribute.getPlungingAttackBaseDamage(0);
    } else if (skill === "e") {
        baseDmg += attribute.getElementalSkillBaseDamage(0);
    } else if (skill === "q") {
        baseDmg += attribute.getElementalBurstBaseDamage(0);
    }

    const dmgWithoutCrit = baseDmg * defensiveRatio * resRatio * damageBonus;

    let crit = skill == "a" ? attribute["critical"] : attribute[skill + "Critical"];
    crit = Math.min(crit, 1);
    crit = Math.max(crit, 0);
    const cd = attribute.criticalDamage;

    return {
        crit: dmgWithoutCrit * (1 + cd),
        nonCrit: dmgWithoutCrit,
        expect: dmgWithoutCrit * (1 + cd * crit),
    };
}


// damage with attack as main component
export function damageNormal(attribute, cLevel, r, enemy, element, skill) {
    return damageDelegate(attribute, cLevel, attribute.attack() * r, enemy, element, skill);
}

// damage with defend as main component
// r: DEF ratio
export function damageDefNormal(attribute, cLevel, r, enemy, element, skill) {
    return damageDelegate(attribute, cLevel, attribute.defend() * r, enemy, element, skill);
}

// damage with baseDmg as main component
export function damageCustom(attribute, cLevel, enemy, element, skill, baseDmg, extraBonus = []) {
    return damageDelegate(attribute, cLevel, baseDmg, enemy, element, skill, extraBonus);
}

export function damageReactionDelegate(type, attribute, base, element) {
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
        crit: base.crit * rate * (1 + amp),
        nonCrit: base.nonCrit * rate * (1 + amp),
        expect: base.expect * rate * (1 + amp),
    };
}


// damage reaction
// type: "melt" | "vaporize"
// element: who triggers the reaction
export function damageReaction(type, attribute, cLevel, r, enemy, element, skill) {
    let baseDmg = damageNormal(attribute, cLevel, r, enemy, element, skill);
    return damageReactionDelegate(type, attribute, baseDmg, element);
}

export function damageReactionCustom(type, attribute, cLevel, enemy, element, skill, baseDmg, extraBonus = []) {
    let base = damageCustom(attribute, cLevel, enemy, element, skill, baseDmg, extraBonus);
    return damageReactionDelegate(type, attribute, base, element);
}

// skillName: a, b, air, e, q
// deprecated
export function colNormal(attribute, configObject, enemy, rowConfigs, skillName, element, as=skillName) {
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
            as,
        ));
    }

    return ret;
}

// deprecated
export function colReaction(type, attribute, configObject, enemy, skillKeys, skillId, element, as=skillId) {
    let c = configObject.character;

    let cName = c.name;
    let skillData = charactersData[cName].skill;

    let skillLevel;
    let skill = skillId;
    if (skillId === "a" || skillId === "b" || skillId === "air") {
        skillLevel = c.skill1;
        skill = "a";
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
            skillData[skill][skillItem.key][skillLevel - 1],
            enemy,
            element,
            as,
        ));
    }

    return ret;
}

export function rowFire(attribute, configObject, enemy, caption, skillName, baseDmg) {
    const cLevel = configObject.character.level;
    return {
        "chs": caption,
        "fire": damageCustom(attribute, cLevel, enemy, "fire", skillName, baseDmg),
        "fireMelt": damageReactionCustom("melt", attribute, cLevel, enemy, "fire", skillName, baseDmg),
        "fireVaporize": damageReactionCustom("vaporize", attribute, cLevel, enemy, "fire", skillName, baseDmg),
    }
}

// deprecated
export function tableFire(attribute, configObject, enemy, rowConfigs, skillName) {
    let ret = mergeArray(
        ["chs", rowConfigs.map(item => item.chs)],
        ["fire", colNormal(attribute, configObject, enemy, rowConfigs, skillName, "fire")],
        ["fireMelt", colReaction("melt", attribute, configObject, enemy, rowConfigs, skillName, "fire")],
        ["fireVaporize", colReaction("vaporize", attribute, configObject, enemy, rowConfigs, skillName, "fire")],
    );
    return ret;
}

// deprecated
export function tableFireCustom(attribute, configObject, enemy, rowConfigs, skillName, baseDmg,
    {extraBonus = []} = {}) {
    let rows = [];

    let cLevel = configObject.character.level;
    
    for (let config of rowConfigs) {
        let row = {
            "chs": config.chs,
            "fire": damageCustom(attribute, cLevel, enemy, "fire", skillName, baseDmg, extraBonus),
            "fireMelt": damageReactionCustom("melt", attribute, cLevel, enemy, "fire", skillName, baseDmg, extraBonus),
            "fireVaporize": damageReactionCustom("vaporize", attribute, cLevel, enemy, "fire", skillName, baseDmg, extraBonus),
        };
        rows.push(row);
    }

    return rows;
}

// deprecated
export function tableIce(attribute, configObject, enemy, rowConfigs, skillName) {
    return mergeArray(
        ["chs", rowConfigs.map(item => item.chs)],
        ["ice", colNormal(attribute, configObject, enemy, rowConfigs, skillName, "ice")],
        ["iceMelt", colReaction("melt", attribute, configObject, enemy, rowConfigs, skillName, "ice")]
    );
}

// deprecated
export function tableThunder(attribute, configObject, enemy, rowConfigs, skillName) {
    return mergeArray(
        ["chs", rowConfigs.map(item => item.chs)],
        ["thunder", colNormal(attribute, configObject, enemy, rowConfigs, skillName, "thunder")],
    );
}

export function rowThunder(attribute, configObject, enemy, caption, skillName, baseDmg) {
    return {
        "chs": caption,
        "thunder": damageCustom(attribute, configObject.character.level, enemy, "thunder", skillName, baseDmg),
    }
}

export function rowWater(attribute, configObject, enemy, caption, skillName, baseDmg) {
    const cLevel = configObject.character.level;
    return {
        "chs": caption,
        "water": damageCustom(attribute, cLevel, enemy, "water", skillName, baseDmg),
        "waterVaporize": damageReactionCustom("vaporize", attribute, cLevel, enemy, "water", skillName, baseDmg)
    }
} 

// deprecated
export function tableWater(attribute, configObject, enemy, rowConfigs, skillName, as=skillName) {
    return mergeArray(
        ["chs", rowConfigs.map(item => item.chs)],
        ["water", colNormal(attribute, configObject, enemy, rowConfigs, skillName, "water", as)],
        ["waterVaporize", colReaction("vaporize", attribute, configObject, enemy, rowConfigs, skillName, "water", as)],
    );
}

// deprecated
export function tableWind(attribute, configObject, enemy, rowConfigs, skillName, as=skillName) {
    return mergeArray(
        ["chs", rowConfigs.map(item => item.chs)],
        ["wind", colNormal(attribute, configObject, enemy, rowConfigs, skillName, "wind", as)],
    )
}

// deprecated
export function tableRock(attribute, configObject, enemy, rowConfigs, skillName, as=skillName) {
    return mergeArray(
        ["chs", rowConfigs.map(item => item.chs)],
        ["rock", colNormal(attribute, configObject, enemy, rowConfigs, skillName, "rock", as)],
    )
}

export function rowRock(attribute, configObject, enemy, caption, skillName, baseDmg,
    { extraBonus = [] } = {}) {
    return {
        "chs": caption,
        "rock": damageCustom(attribute, configObject.character.level, enemy, "rock", skillName, baseDmg, extraBonus),
    }
}

// deprecated
export function tableRockCustom(attribute, configObject, enemy, rowConfigs, skillName, baseDmg,
    { extraBonus = [] } = {})
{
    let rows = [];

    let cLevel = configObject.character.level;
    
    for (let config of rowConfigs) {
        let row = {
            "chs": config.chs,
            "rock": damageCustom(attribute, cLevel, enemy, "rock", skillName, baseDmg, extraBonus),
        };
        rows.push(row);
    }

    return rows;
}

// deprecated
export function tablePhysical(attribute, configObject, enemy, rowConfigs, skillName) {
    let ret =  mergeArray(
        ["chs", rowConfigs.map(item => item.chs)],
        ["normal", colNormal(attribute, configObject, enemy, rowConfigs, skillName, "physical")],
    );
    return ret;
}

export function rowPhysical(attribute, configObject, enemy, caption, skillName, baseDmg) {
    return {
        "chs": caption,
        "normal": damageCustom(attribute, configObject.character.level, enemy, "physical", skillName, baseDmg),
    };
}

export let rowsAir = [
    {
        key: "airDmg1",
        chs: "下坠期间",
    },
    {
        key: "airDmg2",
        chs: "下坠（低空）",
    },
    {
        key: "airDmg3",
        chs: "下坠（高空）",
    }
]