import { charactersData } from "@asset/character";


let skill = charactersData["shanhugongxinhai"].skill;

function f(config) {
    const hasTalent2 = config.character.hasTalent2;

    const skill1 = config.cArgs.skill1;
    const skill3 = config.cArgs.skill3;
    const ratioAtk = (skill.a.dmg1[skill1 - 1] + skill.a.dmg2[skill1 - 1] + skill.a.dmg3[skill1 - 1]) / 3;
    const ratioHpFromQ = skill.q.aBonus[skill3 - 1];

    return function (attribute) {
        const atk = attribute.attack();
        const hp = attribute.life();

        let ratioHp = ratioHpFromQ;
        
        const bonus = attribute.bonus + attribute.waterBonus + attribute.aBonus;
        if (hasTalent2) {
            ratioHp += attribute.cureEffect * 0.15;
        }

        const base = ratioAtk * atk + ratioHp * hp;

        return base * (1 + bonus);
    }
}

export default {
    name: "shanhugongxinhaiQ",
    func: f,
    needConfig: true,
    needContext: false,
}