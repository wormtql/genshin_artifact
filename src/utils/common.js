export function deepCopy(obj) {
    if (Array.isArray(obj)) {
        let temp = [];
        for (let i = 0; i < obj.length; i++) {
            temp.push(deepCopy(obj[i]));
        }
        return temp;
    } else if (typeof obj === "object") {
        let temp = {};
        for (let key in obj) {
            temp[key] = deepCopy(obj[key]);
        }
        return temp;
    }
    return obj;
}

export function convertArtifact(art) {
    let temp = {
        position: art.position,
        setName: art.setName,
        primary: {},
        secondary: {},
        refer: art,
    }

    temp.primary[art.primary.tag] = Number(art.primary.value);
    for (let i = 0; i < art.secondary.length; i++) {
        temp.secondary[art.secondary[i].tag] = Number(art.secondary[i].value);
    }

    return temp;
}

export function recommendAttribute(d) {
    let temp = deepCopy(d);
    temp.sort((a, b) => {
        return b.d - a.d;
    });

    // window.console.log(temp);

    let chs = [];
    for (let i = 0; i < Math.min(3, temp.length); i++) {
        chs.push(temp[i].chs);
    }
    
    return chs;
}