import { artifactsData } from "@asset/artifacts";
import { secondaryTags } from "@asset/tags";

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
    if (!tag.name) {
        return "expecting tag name";
    }
    if (!secondaryTags[tag.name]) {
        return `tag name "${tag.name}" not found`;
    }

    if (!tag.value) {
        return "expecting tag value";
    }
    if (!(typeof tag.value === "number")) {
        return "tag value not right";
    }

    return true;
}

/**
 * check if an artifact is valid
 * @param {*} art 
 * @return a string if error, or true if ok 
 */
function checkArtifact(art) {
    if (!art.setName) {
        return "expecting set name";
    }
    if (!artifactsData[art.setName]) {
        return `artifact name "${art.setName}" not found`;
    }

    if (!art.position) {
        return "expecting position";
    }
    for (let key in alias) {
        for (let a of alias[key]) {
            if (art.position === a) {
                art.position = key;
                break;
            }
        }
    }
    if (positions.indexOf(art.position) === -1) {
        return `position ${art.position} not expected`;
    }

    if (!art.mainTag) {
        return "expecting main tag";
    }
    let chk = checkTag(art.mainTag);
    if (chk !== true) {
        return chk;
    }

    if (!art.normalTags || !Array.isArray(art.normalTags)) {
        return "expecting normal tags";
    }
    for (let tag of art.normalTags) {
        chk = checkTag(tag);
        if (chk !== true) {
            return chk;
        }
    }

    if (!Object.prototype.hasOwnProperty.call(art, "omit")) {
        return "expecting omit";
    }

    if (!Object.prototype.hasOwnProperty.call(art, "id")) {
        return "expecting id";
    }

    return true;
}



export function checkImportJson(str) {
    let json = JSON.parse(str);

    for (let key in alias) {
        alias[key].forEach(name => {
            if (json[name]) {
                json[key] = json[name];
                delete json[name];
            }
        });
        json[key] = json[key] || [];
    }


    positions.forEach(pos => {
        if (json[pos]) {
            if (!Array.isArray(json[pos])) {
                throw new Error(`position "${pos}" is not an array`);
            }
            for (let item of json[pos]) {
                let chk = checkArtifact(item);
                if (typeof chk === "string") {
                    throw new Error(chk);
                }
            }
        } else {
            throw new Error("expecting " + pos);
        }
    })
    
    return json;
}