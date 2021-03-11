import skill from "./skill";

function f(config) {
    let qLevel = config.cArgs.skill3 || 6;
    let bonus = skill.q.hp[qLevel - 1];
    let s = skill.q.s[qLevel - 1];

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