import _data from "./_gen_buff"
// import _data from "!../../loaders/meta_loader.js?type=buff!./meta"
Object.freeze(_data)

export const buffData = _data

let _buffByGenre = {}
for (let name in _data) {
    const buff = _data[name]
    const genre = buff.genre
    if (!Object.prototype.hasOwnProperty.call(_buffByGenre, genre)) {
        _buffByGenre[genre] = []
    }
    _buffByGenre[genre].push(buff)
}
for (let genre in _buffByGenre) {
    _buffByGenre[genre].sort((a, b) => a.chs.localeCompare(b.chs))
}
Object.freeze(_buffByGenre)

export const buffByGenre = _buffByGenre

let _buffFlat = []
for (let name in _data) {
    _buffFlat.push(_data[name])
}
Object.freeze(_buffFlat)

export const buffFlat = _buffFlat
