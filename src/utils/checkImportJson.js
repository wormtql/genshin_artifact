import { artifactsData } from "@asset/artifacts";
import { artifactTags } from "@const/artifact";

const positions = ["flower", "feather", "sand", "cup", "head"];
const alias = {
    flower: [],
    feather: ["plume"],
    sand: ["sands"],
    cup: ["goblet"],
    head: ["circlet"],
}

/**
 * check if a tag is valid
 * @param {*} tag
 * @return a string if error, or true if ok 
 */
function checkTag(tag) {
    // check tag name
    if (!tag.name) {
        return "expecting tag name";
    }
    // check if tag name exists
    if (!artifactTags[tag.name]) {
        return `tag name "${tag.name}" not found`;
    }

    // check tag value
    if (!tag.value) {
        return "expecting tag value";
    }
    // check value type
    if (!(typeof tag.value === "number")) {
        return "tag value not right";
    }

    return true;
}

/**
 * check if an artifact is valid
 * some field will be set to default
 * @param {*} art 
 * @return a string if error, or true if ok 
 */
function checkArtifact(art, doDefault = false) {
    // check set name
    if (!art.setName) {
        return "expecting set name";
    }
    // check if set name exists
    if (!artifactsData[art.setName]) {
        return `artifact name "${art.setName}" not found`;
    }

    // check position
    if (!art.position) {
        return "expecting position";
    }
    // convert aliases
    for (let key in alias) {
        for (let a of alias[key]) {
            if (art.position === a) {
                art.position = key;
                break;
            }
        }
    }
    // check if position exists
    if (positions.indexOf(art.position) === -1) {
        return `position ${art.position} not expected`;
    }

    // check main tag
    if (!art.mainTag) {
        return "expecting main tag";
    }
    // check if tag is valid
    let chk = checkTag(art.mainTag);
    if (chk !== true) {
        return chk;
    }

    // check 副词条
    if (!art.normalTags || !Array.isArray(art.normalTags)) {
        return "expecting normal tags";
    }
    for (let tag of art.normalTags) {
        chk = checkTag(tag);
        if (chk !== true) {
            return chk;
        }
    }

    // check star and level
    if (!Object.prototype.hasOwnProperty.call(art, "star")) {
        if (doDefault) {
            art.star = 5;
        }
    }
    if (!Object.prototype.hasOwnProperty.call(art, "level")) {
        if (doDefault) {
            art.level = 20;
        }
    }
    let star = art.star ?? 5;
    let level = art.level ?? 20;
    if (star < 1 || star > 5) {
        return "star is invalid";
    }
    if (level < 0 || level > star * 4) {
        return "level is invalid";
    }

    // omit default to false
    if (!Object.prototype.hasOwnProperty.call(art, "omit")) {
        if (doDefault) {
            art.omit = false;
        }
    }

    return true;
}



export default function checkImportJson(str) {
    let json = str;
    if (typeof str === "string") {
        json = JSON.parse(str);
    }

    for (let key in alias) {
        alias[key].forEach(name => {
            if (json[name]) {
                json[key] = json[name];
                delete json[name];
            }
        });
        json[key] = json[key] || [];
    }

    let invalidCount = 0;
    let ret = {};
    positions.forEach(pos => {
        ret[pos] = [];
        if (!Array.isArray(json[pos])) {
            throw new Error(`position "${pos}" is not an array`);
        }
        for (let item of json[pos]) {
            let chk = checkArtifact(item);
            if (typeof chk === "string") {
                invalidCount++;
            } else {
                ret[pos].push(item);
            }
        }
    })
    
    return {
        artifacts: ret,
        invalidCount,
    };
}