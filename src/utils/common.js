export function deepCopy(obj) {
    if (Array.isArray(obj)) {
        let temp = [];
        for (let i = 0; i < obj.length; i++) {
            temp.push(deepCopy(obj[i]));
        }
        return temp;
    } else if (typeof obj === "object") {
        if (obj === null) {
            return null
        }

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

export function toSnakeCase(s) {
    let temp = s.replace(/[A-Z]/g, letter => `_${letter.toLowerCase()}`)
    if (temp.startsWith("_")) {
        return temp.substr(1)
    } else {
        return temp
    }
}

// code from https://gist.github.com/danallison/3ec9d5314788b337b682
export function downloadString(text, fileType, filename) {
    const blob = new Blob([text], { type: fileType })

    const a = document.createElement("a")
    a.download = filename
    a.href = URL.createObjectURL(blob)
    a.dataset.downloadurl = [fileType, a.download, a.href].join(":")
    a.style.display = "none"
    document.body.appendChild(a)
    a.click()
    document.body.removeChild(a)
    setTimeout(function() { URL.revokeObjectURL(a.href) }, 1500)
}

export function element2Chs(element) {
    switch (element) {
        case "Pyro":
            return "火"
        case "Hydro":
            return "水"
        case "Electro":
            return "雷"
        case "Anemo":
            return "风"
        case "Cryo":
            return "冰"
        case "Geo":
            return "岩"
        case "Physical":
            return "物理"
        case "Dendro":
            return "草"
    }
}

export function loadScript(url) {
    const removeElement = ele => {
        setTimeout(() => {
            ele.remove()
        }, 0)
    }

    return new Promise((resolve, reject) => {
        const script = document.createElement("script")
        script.src = url
        script.onload = e => {
            removeElement(script)
            resolve(e)
        }
        script.onerror = e => {
            removeElement(script)
            reject(e)
        }
        document.head.appendChild(script)
    })
}
