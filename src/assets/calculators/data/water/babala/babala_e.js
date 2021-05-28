import { tableWater } from "../../../utils";
import { getAttribute } from "@util/attribute";
import { charactersData } from "@asset/characters";

let skill = charactersData["babala"].skill;

let rowsE = [
    {
        key: "dmg1",
        chs: "水珠伤害",
    },
];

export default function (artifacts, configObject, enemy) {
    let c = configObject.character;
    let w = configObject.weapon;
    let attribute = getAttribute(artifacts, c, w, configObject.buffs, configObject.artifactsConfig);

    let idx = c.skill2 - 1;
    let cure1 = attribute.life() * skill.e.cure1[idx] + skill.e.cure1Static[idx];
    let cure2 = attribute.life() * skill.e.cure2[idx] + skill.e.cure2Static[idx];

    let e = tableWater(attribute, configObject, enemy, rowsE, "e");

    return {
        cure1, cure2, e,
    }
}