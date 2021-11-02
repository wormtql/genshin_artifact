import enemyData from "./data.js";

export default class Enemy {
    constructor(name, level) {
        this.name = name;
        this.level = level;
    }

    getDR(characterLevel, defDown = 0) {
        const def = this.level + 100;
        // const defDown = Math.max(defDown, 0);
        const dd = Math.min(defDown, 1);
        return (characterLevel + 100) / ((1 - dd) * def + characterLevel + 100);
    }

    getRR(element, down = 0) {
        let attr;
        if (element === "physical") {
            attr = "phyRes";
        } else {
            attr = element + "Res";
        }

        let res = enemyData[this.name][attr] - down;
        if (res > 0.75) {
            return 25 / (25 + res);
        } else if (res > 0) {
            return 1 - res;
        } else {
            return 1 - res / 2;
        }
    }
}