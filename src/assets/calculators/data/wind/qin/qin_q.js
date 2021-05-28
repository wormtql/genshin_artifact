import { tableWind } from "../../../utils";
import { getAttribute } from "@util/attribute";
import { charactersData } from "@asset/characters";

let skill = charactersData["qin"].skill;

let rowsQ = [
    {
        key: "dmg1",
        chs: "爆发伤害",
    },
    {
        key: "dmg2",
        chs: "出入领域伤害",
    }
];

export default function (artifacts, configObject, enemy) {
    let c = configObject.character;
    let w = configObject.weapon;
    let attribute = getAttribute(artifacts, c, w, configObject.buffs, configObject.artifactsConfig);

    let q = tableWind(attribute, configObject, enemy, rowsQ, "q");
    
    let idx = c.skill3 - 1;
    let cure1 = skill.q.cure1[idx] * attribute.attack() + skill.q.cure1Static[idx];
    let cure2 = skill.q.cure2[idx] * attribute.attack() + skill.q.cure2Static[idx];

    return {
        q, cure1, cure2,
    };
}