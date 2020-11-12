import { supportedCharacters } from "genshin_panel";
import { chsCharacter } from "@/common/chs";

let fire = new Set([
    "diluke",
    "keli",
    "anbo",
    "bannite",
    "xiangling",
    "xinyan",
]);
let water = new Set([
    "mona",
    "dadaliya",
    "babala",
    "xingqiu"
]);
let wind = new Set([
    "qin",
    "wendi",
    "xiao",
    "fengzhu",
    "shatang",
]);
let thunder = new Set([
    "keqing",
    "beidou",
    "lisha",
    "leize",
    "feixieer"
]);
let ice = new Set([
    "qiqi",
    "kaiya",
    "chongyun",
    "diaona",
]);
let rock = new Set([
    "yanzhu",
    "zhongli",
    "nuoaier",
    "ningguang"
]);

let _preset = {
    "thunder": {
        "chs": "雷",
        "characters": {}
    },
    "fire": {
        "chs": "火",
        "characters": {}
    },
    "water": {
        "chs": "水",
        "characters": {}
    },
    "wind": {
        "chs": "风",
        "characters": {}
    },
    "ice": {
        "chs": "冰",
        "characters": {}
    },
    "rock": {
        "chs": "岩",
        "characters": {}
    }
};

function getElement(name) {
    if (fire.has(name)) {
        return "fire";
    }
    if (water.has(name)) {
        return "water";
    }
    if (wind.has(name)) {
        return "wind";
    }
    if (thunder.has(name)) {
        return "thunder";
    }
    if (ice.has(name)) {
        return "ice";
    }
    if (rock.has(name)) {
        return "rock";
    }
}

const support = supportedCharacters();
for (let i = 0; i < support.length; i++) {
    let item = support[i];
    let temp = item.split("-");
    
    let chs = chsCharacter(temp[0]);
    let element = getElement(temp[0]);
    let characters = _preset[element].characters;

    let s1 = chs;
    let s2 = temp[1] + "级";
    let s3 = temp[2] === 0 ? "未突破" : "突破";

    if (chs === null) {
        continue;
    }

    if (characters[temp[0]]) {
        characters[temp[0]].presets.push({
            value: item,
            title: s1 + s2 + s3,
        })
    } else {
        characters[temp[0]] = {
            chs,
            presets: [
                {
                    value: item,
                    title: s1 + s2 + s3,
                }
            ]
        }
    }
}

Object.freeze(_preset);
export const preset = _preset;
