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

export default function (artifacts, configObject, enemy) {
    let c = configObject.character;
    let w = configObject.weapon;
    let attribute = getAttribute(artifacts, c, w, configObject.buffs, configObject.artifactsConfig);

    let q = tableFire(attribute, configObject, enemy, skillKeys, "q");
    let cure1 = skill.q.cure1[c.skill3 - 1] * attribute.life();
    let cure2 = skill.q.cure2[c.skill3 - 1] * attribute.life();

    return {
        q, cure1, cure2,
    };
}