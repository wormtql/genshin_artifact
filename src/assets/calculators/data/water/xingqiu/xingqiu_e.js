import { tableWater } from "../../../utils";
import { getAttribute } from "@util/attribute";
import { charactersData } from "@asset/characters";

let skill = charactersData["xingqiu"].skill;

let rowsE = [
    {
        key: "dmg1",
        chs: "技能伤害-1",
    },
    {
        key: "dmg2",
        chs: "技能伤害-2",
    },
];

export default function (artifacts, configObject, enemy) {
    let c = configObject.character;
    let w = configObject.weapon;
    let attribute = getAttribute(artifacts, c, w, configObject.buffs);

    let idx = c.skill2 - 1;

    let e = tableWater(attribute, configObject, enemy, rowsE, "e");
    let res = skill.e.res[idx] + attribute.waterBonus * 0.2;

    return {
        e, res,
    }
}