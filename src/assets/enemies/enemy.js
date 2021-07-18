import _entries from "./initData";

export default class Enemy {
    constructor(name, level) {
        this.name = name;
        this.level = level;

        let obj = _entries[name];
        this.waterRes = obj.waterRes;
        this.rockRes = obj.rockRes;
        this.iceRes = obj.iceRes;
        this.fireRes = obj.fireRes;
        this.windRes = obj.windRes;
        this.thunderRes = obj.thunderRes;
        this.physicalRes = obj.phyRes;

        this.abyss = obj.abyss;
    }

    // static registerEnemy(name, obj) {
    //     console.log(name);
    //     _entries[name] = obj;
    // }

    getDR(characterLevel, defDown = 0) {
        let def = this.level + 100;
        return (characterLevel + 100) / ((1 - defDown) * def + characterLevel + 100);
    }

    getRR(element) {
        let attr = element + "Res";

        let res = this[attr];
        if (res > 0.75) {
            return 25 / (25 + res);
        } else if (res > 0) {
            return 1 - res;
        } else {
            return 1 - res / 2;
        }
    }

    getProperty(...paths) {
        let ans = this;
        for (let path of paths) {
            let temp = [];
            if (typeof path === "string") {
                temp = path.split(".");
            } else {
                temp = [path];
            }
            
        
            for (let x of temp) {
                ans = ans[x];
            }
        }
        
        return ans;
    }
}

import "./initData";