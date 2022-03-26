import _characterData from "./_gen_character"
// import _characterData from "!../../loaders/meta_loader.js?type=character!./meta"

Object.freeze(_characterData)
export const characterData = _characterData

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
Object.freeze(_charactersByElement)

export const characterByElement = _charactersByElement;