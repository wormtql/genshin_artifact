import attribute from "@const/artifact_eff";
import { random01, randomIntBetween, randomElement } from "@util/random";
import { artifactsSecondaryTag as tags } from "@asset/artifacts";

function randomNormalTag(count, exclude) {
    let canditate = tags.filter(tag => exclude.indexOf(tag) === -1);
    let len = canditate.length;
    let ans = [];
    for (let i = 0; i < count; i++) {
        let index = randomIntBetween(0, len);
        ans.push(canditate[index]);

        let temp = canditate[index];
        canditate[index] = canditate[len - 1];
        canditate[len - 1] = temp;

        len--;
    }
    
    return ans;
}

export default function (star, level, exclude) {
    let attr = attribute[star];

    let upCount = Math.floor(level / 4);
    let initalCount = star - 2 + random01();

    let tagCount = Math.min(4, upCount + initalCount);
    let normalTags = randomNormalTag(tagCount, exclude);
    // console.log(normalTags);

    let ret = [];
    normalTags.forEach(tag => {
        ret.push({
            name: tag,
            value: randomElement(attr[tag]),
        });
    });

    let remain = upCount + initalCount - 4;
    for (let i = 0; i < remain; i++) {
        let index = randomIntBetween(0, ret.length);
        ret[index].value += randomElement(attr[ret[index].name]);
    }
    // console.log(ret);

    return ret;
}