import { tableFire } from "../../../utils";
import { getAttribute } from "@util/attribute";
import { charactersData } from "@asset/characters";

const skill = charactersData["hutao"].skill;

let skillKeys = [
    {
        key: "dmg1",
        chs: "伤害",
    },
    {
        key: "dmg2",
        chs: "低血量伤害",
    },
];

export default function (artifacts, configObject, enemy, otherConfig) {
    let c = configObject.character;
    let w = configObject.weapon;
    let attribute = getAttribute(artifacts, c, w, configObject.buffs, configObject.artifactsConfig);

    if (otherConfig.afterE) {
        let atkBonus = Math.min(4 * attribute.attackBasic, skill.e.hp[c.skill2 - 1] * attribute.life());
        attribute.attackStatic += atkBonus;
    }

    let q = tableFire(attribute, configObject, enemy, skillKeys, "q");
    let cure1 = skill.q.cure1[c.skill3 - 1] * attribute.life();
    let cure2 = skill.q.cure2[c.skill3 - 1] * attribute.life();

    return {
        q, cure1, cure2,
    };
}