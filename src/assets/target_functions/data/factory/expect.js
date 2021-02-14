function helper(element) {
    return function(attribute) {
        let atk = attribute.attack();
        let bonus = attribute.bonus + attribute.aBonus + attribute[element + "Bonus"];
        let crit = Math.min(attribute.critical, 1);

        let baseDmg = atk * (1 + bonus);
        return (crit * attribute.criticalDamage + 1) * baseDmg;
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