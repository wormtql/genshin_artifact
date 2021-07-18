const computeShieldRate = require("../src/abyss").computeShieldRate;
const Enemy = require("../src/assets/enemies/enemy");

let enemiesConfig = [
    { name: "hilichurl" },
    { name: "anemoSamachurl" },
    { name: "rockShieldwallMitachurl" },
    { name: "iceShieldMitachurl" },
]

let enemies = enemiesConfig.map(item => {
    return new Enemy(item.name, 100);
})

console.log(enemies);