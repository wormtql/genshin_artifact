import skill from "./skill";

function f(config) {
    let qLevel = config.cArgs.skill3;
    let bonus = skill.q.atk[qLevel - 1];
    let s = skill.q.s[qLevel - 1];

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