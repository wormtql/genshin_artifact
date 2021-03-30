function isMainTagSame(a, b) {
    if (a.mainTag.name !== b.mainTag.name) {
        return false;
    }

    if (a.mainTag.value !== b.mainTag.value) {
        return false;
    }

    return true;
}

function isNormalTagsSame(a, b) {
    if (a.normalTags.length !== b.normalTags.length) {
        return false;
    }

    let h = new Set();
    for (let tag of a.normalTags) {
        let hash = tag.name + tag.value.toFixed(3);
        h.add(hash);
    }

    for (let tag of b.normalTags) {
        let hash = tag.name + tag.value.toFixed(3);
        if (!h.has(hash)) {
            return false;
        }
    }

    return true;
}

function isSame(a, b) {
    if (a.setName !== b.setName) {
        return false;
    }

    if (a.position !== b.position) {
        return false;
    }

    if (!isMainTagSame(a, b)) {
        return false;
    }

    if (!isNormalTagsSame(a, b)) {
        return false;
    }

    return true;
}

export default function (all, artifact) {
    for (let art of all) {
        if (isSame(art, artifact)) {
            // console.log(art, artifact);
            return false;
        }
    }

    return true;
}