import { tablePhysical,tableFire } from "../../../utils";
import { getAttribute } from "@util/attribute";
import { charactersData } from "@asset/characters";

const skill = charactersData["hutao"].skill;

let skillKeys = [
    {
        key: "xuemeixiang",
        chs: "血梅香",
    },
];

export default function (artifacts, configObject, enemy, otherConfig) {
    let c = configObject.character;
    let w = configObject.weapon;
    let attribute = getAttribute(artifacts, c, w, configObject.buffs, configObject.artifactsConfig);

    let f = otherConfig.afterE ? tableFire : tablePhysical;
    if (otherConfig.afterE) {
        atkBonus = Math.min(4 * attribute.attackBasic, skill.e.hp[c.skill2 - 1] * attribute.life());
        attribute.attackStatic += atkBonus;
        e = f(attribute, configObject, enemy, skillKeys, "e");
    }

    let atkBonus = Math.min(4 * attribute.attackBasic, skill.e.hp[c.skill2 - 1] * attribute.life());
    let e = tableFire(attribute, configObject, enemy, skillKeys, "e");

    return {
        atkBonus, e,
    }
}