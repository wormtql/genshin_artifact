import { tableWater } from "../../../utils";
import { getAttribute } from "@util/attribute";
import { charactersData } from "@asset/characters";

let skill = charactersData["mona"].skill;

let rowsQ = [
    {
        key: "dmg1",
        chs: "泡影破裂伤害",
    },
];

export default function (artifacts, configObject, enemy) {
    let c = configObject.character;
    let w = configObject.weapon;
    let attribute = getAttribute(artifacts, c, w, configObject.buffs, configObject.artifactsConfig);

    let q = tableWater(attribute, configObject, enemy, rowsQ, "q");
    let bonus = skill.q.bonus[c.skill3 - 1];

    return {
        q, bonus,
    }
}