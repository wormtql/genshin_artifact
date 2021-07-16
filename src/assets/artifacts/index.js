import _artifactsTagMap from "./artifacts_main_tag.json";
import _artifactsSecondaryTag from "./artifacts_secondary_tag.json";
import _artifactsIcon from "./icons";

let cache = {};

function importAll(r) {
    for (let path of r.keys()) {
        let name = r(path).default.eng;
        cache[name] = r(path).default;
    }
}

importAll(require.context("./data", true, /index\.js$/));

export const artifactsData = cache;
export const artifactsTagMap = _artifactsTagMap;
export const artifactsSecondaryTag = _artifactsSecondaryTag;
export const artifactsIcon = _artifactsIcon;