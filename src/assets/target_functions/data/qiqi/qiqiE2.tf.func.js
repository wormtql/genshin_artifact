import skill from "./skill";

function f(config) {
    let eLevel = config.cArgs.skill2;
    let bonus = skill.e.atk2[eLevel - 1];
    let s = skill.e.s2[eLevel - 1];

    return function (attribute) {
        let atk = attribute.attack();
        return (atk * bonus + s) * (1 + attribute.cureEffect);
    }
}

export default {
    name: "qiqiE2",
    func: f,
    needConfig: true,
}