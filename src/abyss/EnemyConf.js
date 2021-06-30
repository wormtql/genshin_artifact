import enemiesData from "@asset/enemies/data";

export default class EnemyConf {
    constructor(enemies) {
        this.shield = {
            fire: 0,
            ice: 0,
            thunder: 0,
            water: 0,
            rock: 0,
            wind: 0,
            grass: 0,
        };
        this.group = false;
        this.element = {
            fire: 0,
            ice: 0,
            thunder: 0,
            water: 0,
            rock: 0,
            wind: 0,
            grass: 0,
        }

        for (let enemy of enemies) {
            let name = enemy.name;
            let data = enemiesData[name];
            let count = enemy.count;
            if (data.shield) {
                for (let shield of data.shield) {
                    
                }
            }
        }
    }
}