// import attribute from "@const/artifact_eff";
import { random01, randomIntBetween, randomElement } from "@util/random";
// import { artifactsSecondaryTag as tags } from "@asset/artifacts";
import {artifactTags as tags, subStats, artifactEff} from "@/constants/artifact"

function randomNormalTag(count, exclude) {
    let candidate = subStats.filter(tag => exclude.indexOf(tag) === -1);
    let len = candidate.length;
    let ans = [];
    for (let i = 0; i < count; i++) {
        let index = randomIntBetween(0, len);
        ans.push(candidate[index]);

        let temp = candidate[index];
        candidate[index] = candidate[len - 1];
        candidate[len - 1] = temp;

        len--;
    }
    
    return ans;
}

export default function (star, level, exclude) {
    let attr = artifactEff[star];

    let upCount = Math.floor(level / 4);
    let initialCount = star - 2 + random01();

    let tagCount = Math.min(4, upCount + initialCount);
    let normalTags = randomNormalTag(tagCount, exclude);
    // console.log(normalTags);

    let ret = [];
    normalTags.forEach(tag => {
        ret.push({
            name: tag,
            value: randomElement(attr[tag]),
        });
    });

    let remain = upCount + initialCount - 4;
    for (let i = 0; i < remain; i++) {
        let index = randomIntBetween(0, ret.length);
        ret[index].value += randomElement(attr[ret[index].name]);
    }
    // console.log(ret);

    return ret;
}