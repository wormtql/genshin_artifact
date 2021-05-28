import { tableThunder, damageNormal } from "../../../utils";
import { getAttribute } from "@util/attribute";
import { charactersData } from "@asset/characters";
import mergeArray from "@util/mergeArray";


let skill = charactersData["leize"].skill;

let rowsQ = [
    {
        key: "dmg1",
        chs: "爆发伤害",
    },
];

let rowsA = [
    {
        key: "dmg1",
        chs: "雷狼普攻1段",
    },
    {
        key: "dmg2",
        chs: "雷狼普攻2段",
    },
    {
        key: "dmg3",
        chs: "雷狼普攻3段",
    },
    {
        key: "dmg4",
        chs: "雷狼普攻4段",
    },
];

function col(attribute, enemy, configObject, rowConfigs, ratio) {
    let c = configObject.character;
    let level = c.level;
    let skillLevel = c.skill1;

    let ret = [];
    for (let i = 0; i < rowConfigs.length; i++) {
        let skillItem = rowConfigs[i];
        ret.push(damageNormal(
            attribute,
            level,
            skill.a[skillItem.key][skillLevel - 1] * ratio,
            enemy,
            "thunder",
            "a",
        ));
    }

    return ret;
}

export default function (artifacts, configObject, enemy) {
    let c = configObject.character;
    let w = configObject.weapon;
    let attribute = getAttribute(artifacts, c, w, configObject.buffs, configObject.artifactsConfig);

    let q = tableThunder(attribute, configObject, enemy, rowsQ, "q");

    let ratioWolf = skill.q.dmg2[c.skill3 - 1];
    let wolf = mergeArray(
        ["chs", rowsA.map(item => item.chs)],
        ["thunder", col(attribute, enemy, configObject, rowsA, ratioWolf)],
    );

    return {
        q, wolf,
    }
}