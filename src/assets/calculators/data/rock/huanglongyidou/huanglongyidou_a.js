import { rowPhysical, rowRock, rowsAir } from "../../../utils";
import { getAttribute } from "@util/attribute";
import { charactersData } from "@asset/characters";


const skill = charactersData["huanglongyidou"].skill;

const rowsA = [
    { key: "dmg1", chs: "一段伤害" },
    { key: "dmg2", chs: "二段伤害" },
    { key: "dmg3", chs: "三段伤害" },
    { key: "dmg4", chs: "四段伤害" },
];

const rowsB1 = [
    { key: "bDmg1", chs: "荒泷逆袈裟连斩伤害" },
    { key: "bDmg2", chs: "荒泷逆袈裟终结伤害" },
]

const rowsB2 = [
    { key: "bDmg3", chs: "左一文字斩伤害" },
]

export default function (artifacts, configObject, enemy, { afterQ }) {
    let c = configObject.character;
    let w = configObject.weapon;
    let attribute = getAttribute(artifacts, c, w, configObject.buffs, configObject.artifactsConfig);

    const hasTalent2 = c.level > 60 || (c.level === 60 && c.ascend);

    let f = rowPhysical;
    if (afterQ) {
        attribute.attackPercentage += attribute.defend() * skill.q.atkBonus[c.skill3 - 1];
        f = rowRock;
    }

    let a = [];
    for (let row of rowsA) {
        const atk = attribute.attack();
        const base = atk * skill.a[row.key][c.skill1 - 1];
        a.push(f(attribute, configObject, enemy, row.chs, "a", base));
    }

    let b = [];
    const gesaBonus = hasTalent2 ? 0.35 : 0;

    attribute.bDefRatio += gesaBonus;
    for (let row of rowsB1) {
        const atk = attribute.attack();
        const base = atk * skill.a[row.key][c.skill1 - 1];
        b.push(f(attribute, configObject, enemy, row.chs, "b", base));
    }
    attribute.bDefRatio -= gesaBonus;

    for (let row of rowsB2) {
        const atk = attribute.attack();
        const base = atk * skill.a[row.key][c.skill1 - 1];
        b.push(f(attribute, configObject, enemy, row.chs, "b", base));
    }

    let air = [];
    for (let row of rowsAir) {
        const atk = attribute.attack();
        const base = atk * skill.a[row.key][c.skill1 - 1];
        air.push(f(attribute, configObject, enemy, row.chs, "air", base));
    }

    return {
        a, b, air,
    }
}