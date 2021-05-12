// import * as genshin from "genshin_panel";

import { tableNormalA, tableReactionA } from "../../utils";
import mergeArray from "@util/mergeArray";
import { getAttribute } from "@util/attribute";
// import { charactersData } from "@asset/characters";

// let hutaoSkill = charactersData["hutao"].skill;

let skillKeys = [
    {
        key: "xuemeixiang",
        chs: "血梅香",
        skill: "e",
        element: "fire",
    },
];

export default function (artifacts, configObject, enemy) {
    let c = configObject.character;
    let w = configObject.weapon;
    let attribute = getAttribute(artifacts, c, w, configObject.buffs);
    // let weapon = new genshin.Weapon(w.name, w.level, w.ascend, w.refine, w.args);

    let ret = mergeArray(
        ["chs", skillKeys.map(item => item.chs)],
        ["xmx", tableNormalA(attribute, configObject, enemy, skillKeys, "e")],
        ["xmxMelt", tableReactionA("melt", attribute, configObject, enemy, skillKeys, "e")],
        ["xmxVaporize", tableReactionA("vaporize", attribute, configObject, enemy, skillKeys, "e")],
    );
    // console.log(ret);

    // let baseAtk = weapon.baseAtk;
    // let bonusAtk = Math.min(4 * baseAtk, hutaoSkill.e.hp[c.skill2 - 1] * attribute.life());

    // ret.push({
    //     chs: "提升攻击力",
    //     bonusAtk,
    // })

    return ret;
}