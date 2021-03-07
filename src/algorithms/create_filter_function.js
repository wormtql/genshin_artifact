import { IDENTITY } from "@util/functions";

export default function (config) {
    config = config.constraintMainTag;

    if (config.sand === "any" && config.cup === "any" && config.head === "any") {
        return IDENTITY;
    }

    return function (artifacts) {
        let ret = {
            flower: artifacts.flower,
            feather: artifacts.feather,
        };

        if (config.sand === "any") {
            ret.sand = artifacts.sand;
        } else {
            ret.sand = artifacts.sand.filter(item => item.mainTag.name === config.sand);
        }
        
        if (config.cup === "any") {
            ret.cup = artifacts.cup;
        } else {
            ret.cup = artifacts.cup.filter(item => item.mainTag.name === config.cup);
        }

        if (config.head === "any") {
            ret.head = artifacts.head;
        } else {
            ret.head = artifacts.head.filter(item => item.mainTag.name === config.head);
        }

        return ret;
    }
}