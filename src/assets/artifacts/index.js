import _artifactsTagMap from "./artifacts_main_tag.json";
import _artifactsSecondaryTag from "./artifacts_secondary_tag.json";
import _artifactsIcon from "./icons";

let cache = {};

function importAll(r) {
    for (let path of r.keys()) {
        let name = path.split("/")[1];
        cache[name] = r(path).default;
    }
}

importAll(require.context("./data", true, /index\.js$/));

export const artifactsData = cache;
export const artifactsTagMap = _artifactsTagMap;
export const artifactsSecondaryTag = _artifactsSecondaryTag;
export const artifactsIcon = _artifactsIcon;

export const artifactsNamesSet = new Set();
export const artifactsNameTypeMap = {};

(() => {
    for (const artifact of Object.values(cache)) {
        if (artifact.flower) {
            artifactsNamesSet.add(artifact.flower.chs);
            artifactsNameTypeMap[artifact.flower.chs] = artifact.eng;
        }
        if (artifact.feather) {
            artifactsNamesSet.add(artifact.feather.chs);
            artifactsNameTypeMap[artifact.feather.chs] = artifact.eng;
        }
        if (artifact.sand) {
            artifactsNamesSet.add(artifact.sand.chs);
            artifactsNameTypeMap[artifact.sand.chs] = artifact.eng;
        }
        if (artifact.cup) {
            artifactsNamesSet.add(artifact.cup.chs);
            artifactsNameTypeMap[artifact.cup.chs] = artifact.eng;
        }
        if (artifact.head) {
            artifactsNamesSet.add(artifact.head.chs);
            artifactsNameTypeMap[artifact.head.chs] = artifact.eng;
        }
    }
})();

export const artifactsPositionTypeMap = {
    '生之花': 'flower',
    '死之羽': 'feather',
    '时之沙': 'sand',
    '空之杯': 'cup',
    '理之冠': 'head',
}
