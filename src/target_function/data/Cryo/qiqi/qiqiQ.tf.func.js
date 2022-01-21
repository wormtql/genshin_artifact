import { charactersData } from "@asset/characters";


let skill = charactersData["qiqi"].skill;

function f(config) {
    let qLevel = config.cArgs.skill3;
    let bonus = skill.q.cure1[qLevel - 1];
    let s = skill.q.cure1Static[qLevel - 1];

    return function (attribute) {
        let atk = attribute.attack();
        return (atk * bonus + s) * (1 + attribute.cureEffect);
    }
}

export default {
    name: "qiqiQ",
    func: f,
    needConfig: true,
}