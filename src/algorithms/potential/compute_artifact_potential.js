import createPotentialFunc from "./create_potential_func";
import { getArtifactUpCount } from "@util/utils";

import artifactNumeric from "@const/artifact_eff";

function helper(tags, dep, valid, func, star) {
    // let currentValue = func(tags);
    let value = 0;
    // let star = 5;

    if (dep === 0) {
        return func(tags.slice(1));
    }

    // includes main tag
    if (tags.length < 5) {
        let validProb = 0;
        let already = tags.map(t => t.name);
        // tags that does not currently present
        let valid2 = valid.filter(v => already.indexOf(v) === -1);

        let singleProb = 0.25 / (10 - already.length);
        for (let i = 0; i < 4; i++) {
            for (let validTag of valid2) {
                tags.push({ name: validTag, value: artifactNumeric[star][validTag][i] });
                value += singleProb * helper(tags, dep - 1, valid, func, star);
                tags.pop();
                validProb += singleProb;
            }
        }
        // push an blank tag
        tags.push({ name: "blank", value: 0 });
        value += (1 - validProb) * helper(tags, dep - 1, valid, func, star);

        return value;
    }

    let validProb = 0;
    for (let i = 1; i < 5; i++) {
        let name = tags[i].name;
        if (valid.indexOf(name) !== -1) {
            // if is a valid tag

            let bk = tags[i].value;
            for (let j = 0; j < 4; j++) {
                tags[i].value += artifactNumeric[star][name][j];
                value += 0.0625 * helper(tags, dep - 1, valid, func, star);
                validProb += 0.0625;
                tags[i].value = bk;
            }
        }
    }
    value += (1 - validProb) * helper(tags, dep - 1, valid, func, star);
    return value;
}

export default function (artifact, pfName, pArgs) {
    let star = artifact.star ?? 5;
    if (star <= 3) {
        return -1;
    }

    let f = createPotentialFunc(pfName, pArgs);

    if (pArgs.initialMode) {
        return f.f(artifact);
    }

    let upCount = getArtifactUpCount(artifact);
    // console.log(upCount);

    let tags = [];
    tags.push(artifact.mainTag);
    tags = tags.concat(artifact.normalTags);

    return helper(tags, upCount, f.valid, f.f, star);
}
