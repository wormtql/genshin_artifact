import _artifactsTagMap from "./artifacts_main_tag.json";
import _artifactsSecondaryTag from "./artifacts_secondary_tag.json";
import _artifactsIcon from "./icons";

// let cache = {};

// function importAll(r) {
//     for (let path of r.keys()) {
//         let name = r(path).default.eng;
//         cache[name] = r(path).default;
//     }
// }

// importAll(require.context("./data", true, /index\.js$/));

// import _data from "!../../../loaders/meta_loader.js?type=artifact!../meta"
import _data from "../_gen_artifact"

export const artifactsData = _data;
// export const artifactsData = cache;
export const artifactsTagMap = _artifactsTagMap;
export const artifactsSecondaryTag = _artifactsSecondaryTag;
export const artifactsIcon = _artifactsIcon;