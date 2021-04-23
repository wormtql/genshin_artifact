import { IDENTITY } from "@util/functions";

function createFilterMainTag(config) {
    if (!config) {
        return IDENTITY;
    }
    if (Object.keys(config).length === 0) {
        return IDENTITY;
    }
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

function createFilterLevel(config) {
    if (!config) {
        return IDENTITY;
    }
    if (Object.keys(config).length === 0) {
        return IDENTITY;
    }

    let helper = artifact => {
        // for historical reasons, artifact may not have "level" attribute
        if (!Object.prototype.hasOwnProperty.call(artifact, "level")) {
            return true;
        }
        return artifact.level >= config.min && artifact.level <= config.max;
    }

    return function (artifacts) {
        let temp = {
            flower: artifacts.flower.filter(helper),
            feather: artifacts.feather.filter(helper),
            sand: artifacts.sand.filter(helper),
            cup: artifacts.cup.filter(helper),
            head: artifacts.head.filter(helper),
        };

        return temp;
    }
}

export default function (config) {
    let filterMainTagConfig = config.constraintMainTag ?? {};
    let filterLevelConfig = config.filterLevel ?? {};

    let filterMainTagFunc = createFilterMainTag(filterMainTagConfig);
    let filterLevelFunc = createFilterLevel(filterLevelConfig);

    return function (artifacts) {
        let temp = filterMainTagFunc(artifacts);
        temp = filterLevelFunc(temp);

        return temp;
    }
}