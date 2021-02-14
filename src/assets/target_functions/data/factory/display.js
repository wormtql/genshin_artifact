// 使得单次攻击的伤害上限最高

function helper(element) {
    return function(attribute) {
        let atk = attribute.attack();
        let bonus = attribute.bonus + attribute.aBonus + attribute[element + "Bonus"];

        let baseDmg = atk * (1 + bonus);
        return (1 + attribute.criticalDamage) * baseDmg;
    };
}

let fire = helper("fire");
let thunder = helper("thunder");
let ice = helper("ice");
let wind = helper("wind");
let rock = helper("rock");
let water = helper("water");
let physical = helper("physical");

let temp = {
    fire,
    thunder,
    ice,
    wind,
    rock,
    water,
    physical,
};

export default function(element) {
    return temp[element];
}