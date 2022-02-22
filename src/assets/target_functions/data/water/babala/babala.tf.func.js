import { charactersData } from "@asset/character";


let skill = charactersData["babala"].skill;

function f(config) {
    let qLevel = config.cArgs.skill3 || 6;
    let bonus = skill.q.cure1[qLevel - 1];
    let s = skill.q.cure1Static[qLevel - 1];

    return function (attribute) {
        let hp = attribute.life();
        return (hp * bonus + s) * (1 + attribute.cureEffect);
    };
}

export default {
    name: "babalaQ",
    func: f,
    needConfig: true,
}