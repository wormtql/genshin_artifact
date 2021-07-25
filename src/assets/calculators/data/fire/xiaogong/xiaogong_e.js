import { damageReactionCustom, damageCustom } from "../../../utils";
import { getAttribute } from "@util/attribute";
import { charactersData } from "@asset/characters";

let yoimiyaSkill = charactersData["xiaogong"].skill;

let rows = [
    { key: "dmg1", chs: "普攻1段/2" },
    { key: "dmg2", chs: "普攻2段" },
    { key: "dmg3", chs: "普攻3段" },
    { key: "dmg4", chs: "普攻4段/2" },
    { key: "dmg5", chs: "普攻5段" },
];

function rowFire(attribute, configObject, enemy, rowConfig) {
    let cLevel = configObject.character.level;
    let skill1 = configObject.character.skill1;
    let skill2 = configObject.character.skill2;
    let ratio = yoimiyaSkill.a[rowConfig.key][skill1 - 1] * yoimiyaSkill.e.dmg1[skill2 - 1];
    let baseDmg = ratio * attribute.attack();

    let fire = damageCustom(attribute, cLevel, enemy, "fire", "a", baseDmg);
    let fireMelt = damageReactionCustom("melt", attribute, cLevel, enemy, "fire", "a", baseDmg);
    let fireVaporize = damageReactionCustom("vaporize", attribute, cLevel, enemy, "fire", "a", baseDmg);

    return {
        fire, fireMelt, fireVaporize,
        chs: rowConfig.chs,
    }
}

export default function (artifacts, configObject, enemy) {
    let c = configObject.character;
    let w = configObject.weapon;
    let attribute = getAttribute(artifacts, c, w, configObject.buffs, configObject.artifactsConfig);

    let table = rows.map(rowConfig => {
        return rowFire(attribute, configObject, enemy, rowConfig);
    });

    return {
        e: table,
    };
}