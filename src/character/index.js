function importCharacterData(r) {
    let temp = {}
    for (let path of r.keys()) {
        let data = r(path).default
        temp[data.name] = data
    }
    return temp
}

export const characterData = importCharacterData(require.context("./data", true, /index\.js$/));

function importConfigs(r) {
    let configs = {}
    for (let path of r.keys()) {
        let im = r(path)
        let characterName = im.characterName
        configs[characterName] = im.default
    }

    return configs
}

export const characterConfig = importConfigs(
    require.context("./data", true, /\.ccfg\.yaml$/)
)

export const characterSkillConfig = importConfigs(
    require.context("./data", true, /\.cscfg\.yaml$/)
)

let _charactersByElement = {
    Pyro: [],
    Cryo: [],
    Anemo: [],
    // grass: [],
    Electro: [],
    Hydro: [],
    Geo: [],
}
for (let character of Object.values(characterData)) {
    if (Object.prototype.hasOwnProperty.call(_charactersByElement, character.element)) {
        _charactersByElement[character.element].push(character)
    } else {
        console.log(character)
    }
}
for (let ele in _charactersByElement) {
    _charactersByElement[ele].sort((a, b) => {
        return b.star - a.star;
    })
}

export const characterByElement = _charactersByElement;