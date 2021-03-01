import badge from "../badges/thunder_slime.png";
import config from "./singleConfig";


function f(config) {
    let name = config.tArgs.fieldName;

    if (name === "attack") {
        return function (attribute) {
            return attribute.attack();
        }
    }
    
    if (name === "life") {
        return function (attribute) {
            return attribute.life();
        }
    }

    if (name === "defend") {
        return function (attribute) {
            return attribute.defend();
        }
    }

    return function(attribute) {
        return attribute[name];
    };
}

export default {
    name: "single",
    chs: "单个属性",
    description: [
        "使得某个属性最大"
    ],
    tags: [
        "单值",
    ],
    func: f,
    "for": "common",
    badge,
    needConfig: true,
    config,
}