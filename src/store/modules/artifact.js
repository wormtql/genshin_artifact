import Vue from "vue";

import { hash as hashArtifact } from "@util/artifacts";
import positions from "@const/positions";
import { deepCopy } from "@util/common"

// id can only be changed in store mutations
let id = 0;

let flower = [];
let feather = [];
let sand = [];
let cup = [];
let head = [];
let hashes = new Map();
let localStoredArtifacts = localStorage.getItem("artifacts");
if (localStoredArtifacts) {
    let obj = JSON.parse(localStoredArtifacts);

    flower = obj.flower || [];
    feather = obj.feather || [];
    sand = obj.sand || [];
    cup = obj.cup || [];
    head = obj.head || [];

    let temp = flower.concat(feather).concat(sand).concat(cup).concat(head);
    for (let item of temp) {

        id = Math.max(id, item.id ?? -1);
        const hash = hashArtifact(item);
        if (hashes.has(hash)) {
            hashes.set(hash, hashes.get(hash) + 1);
        } else {
            hashes.set(hash, 1);
        }
    }
    id++;
    for (let item of temp) {
        if (!Object.prototype.hasOwnProperty.call(item, "id")) {
            item.id = id++;
        }
    }
}

function findArtifact(state, id) {
    for (let pos of positions) {
        let arr = state[pos];
        for (let art of arr) {
            if (art.id === id) {
                return art;
            }
        }
    }

    throw new Error("id not found");
}

function updateHash(hash, inc) {
    if (hashes.has(hash)) {
        hashes.set(hash, hashes.get(hash) + inc);
        if (hashes.get(hash) === 0) {
            hashes.delete(hash);
        }
    } else {
        hashes.set(hash, inc);
    }
}

function count(artifacts) {
    let c = 0;

    positions.forEach(pos => {
        c += artifacts[pos].length;
    });

    return c;
}

function removeArtifact(state, id) {
    a: for (let pos of positions) {
        let artifacts = state[pos];
        for (let i = 0, l = artifacts.length; i < l; i++) {
            let art = artifacts[i];
            if (art.id === id) {
                const hash = hashArtifact(art);
                updateHash(hash, -1);
                artifacts.splice(i, 1);
                break a;
            }
        }
    }
}

function addArtifact(state, artifact, fixedId = -1) {
    const id = fixedId !== -1 ? fixedId : Math.floor(Math.random() * 1e9)

    let artifact2 = deepCopy(artifact)
    artifact2.id = id
    if (!Object.hasOwnProperty.call(artifact2, "omit")) {
        artifact2.omit = false
    }

    const position = artifact2.position
    state[position].push(artifact2)

    const hash = hashArtifact(artifact2)
    updateHash(hash, 1)
}

let _store = {
    namespaced: true,
    state: {
        flower,
        feather,
        sand,
        cup,
        head,
    },
    mutations: {
        removeArtifactById(state, { id }) {
            removeArtifact(state, id)
        },

        updateArtifact(state, { id, artifact }) {
            let original = findArtifact(state, id)

            // console.log(original)
            if (original.position !== artifact.position) {
                removeArtifact(state, id)
                addArtifact(state, artifact, id)
            } else {
                for (const key of Object.keys(artifact)) {
                    Vue.set(original, key, artifact[key])
                }
            }

            // const check = name => Object.prototype.hasOwnProperty.call(artifact, name)
            // const attributes = ["setName", "star", "level", "position", "mainTag", "normalTags"]
            //
            // for (let a of attributes) {
            //     if (check(a)) {
            //         Vue.set(original, a, artifact[a])
            //     }
            // }
        },

        addArtifact(state, item) {
            addArtifact(state, item)
        },

        addArtifactV2(state, { artifact }) {
            addArtifact(state, artifact)
            // let item = deepCopy(artifact)
            // item.id = Math.floor(Math.random() * 1e9)
            //
            // if (!Object.hasOwnProperty.call(item, "omit")) {
            //     item.omit = false
            // }
            //
            // state[item.position].push(item)
        },

        addArtifactWithID(state, item) {
            state[item.position].push(item);

            const hash = hashArtifact(item);
            updateHash(hash, 1);
        },

        toggleArtifact(state, obj) {
            let art = state[obj.position][obj.index];
            art.omit = !art.omit;
        },

        toggleArtifactById(state, { id }) {
            a: for (let pos of positions) {
                for (let art of state[pos]) {
                    if (art.id === id) {
                        Vue.set(art, "omit", !art.omit);
                        break a;
                    }
                }
            }
        },

        toggleById(state, payload) {
            let art = findArtifact(state, payload.id);
            art.omit = !art.omit;
        },

        disableArtifactById(state, obj) {
            let id = obj.id;
            positions.forEach(item => {
                let arr = state[item];
                for (let art of arr) {
                    if (art.id === id) {
                        art.omit = true;
                    }
                }
            })
        },

        enableArtifactById(state, { id }) {
            positions.forEach(item => {
                let arr = state[item];
                for (let art of arr) {
                    if (art.id === id) {
                        art.omit = false;
                        return;
                    }
                }
            });
        },

        unlockAll(state) {
            positions.forEach(pos => {
                for (let art of state[pos]) {
                    art.omit = false;
                }
            })
        },

        /**
         * set a single artifact
         */
        setArtifact(state, { artifact, position, index }) {
            const oldHash = hashArtifact(state[position][index]);
            updateHash(oldHash, -1);

            const newHash = hashArtifact(artifact);
            updateHash(newHash, 1);

            Vue.set(state[position], index, artifact);
        },

        setArtifactById(state, { id, newArt }) {
            a: for (let pos of positions) {
                let artifacts = state[pos];
                for (let i = 0, l = artifacts.length; i < l; i++) {
                    let art = artifacts[i];
                    if (art.id === id) {
                        const oldHash = hashArtifact(art);
                        updateHash(oldHash, -1);

                        const newHash = hashArtifact(newArt);
                        updateHash(newHash, 1);

                        newArt.id = id;
                        artifacts.splice(i, 1, newArt);
                        break a;
                    }
                }
            }
        },

        // appendArtifacts(state, obj) {
        //     positions.forEach(pos => {
        //         for (let art of obj[pos]) {
        //             art.id = id++;
        //             state[pos].push(art);
        //
        //             const hash = hashArtifact(art);
        //             updateHash(hash, 1);
        //         }
        //     })
        // },

        appendArtifactsCheckHash(state, obj) {
            for (let pos of positions) {
                let posArtifacts = obj[pos];
                for (let art of posArtifacts) {
                    const hashNew = hashArtifact(art);
                    if (hashes.has(hashNew)) {
                        continue;
                    } else {
                        updateHash(hashNew, 1);
                        art.id = id++;
                        state[pos].push(art);
                    }
                    // console.log(hashNew);
                }
            }
        },

        removeAllArtifacts(state) {
            state.flower = [];
            state.feather = [];
            state.sand = [];
            state.cup = [];
            state.head = [];

            hashes.clear();
        }
    },
    getters: {
        notOmittedArtifacts: state => {
            let fil = item => !item.omit;

            return {
                flower: state.flower.filter(fil),
                feather: state.feather.filter(fil),
                sand: state.sand.filter(fil),
                cup: state.cup.filter(fil),
                head: state.head.filter(fil),
            };
        },

        allArtifacts: state => {
            return {
                flower: state.flower,
                feather: state.feather,
                sand: state.sand,
                cup: state.cup,
                head: state.head,
            };
        },

        allFlat: state => {
            let temp = [];
            positions.forEach(pos => temp = temp.concat(state[pos]));

            return temp;
        },

        artifactsById: state => {
            let temp = {};
            positions.forEach(pos => {
                for (let art of state[pos]) {
                    temp[art.id] = art;
                }
            });

            return temp;
        },

        twentyCount: (state, getters) => {
            let count = 0
            for (let artifact of getters.allFlat) {
                const level = artifact.level ?? 0
                if (level === 20) {
                    count += 1
                }
            }
            return count
        },

        flowerCount: state => {
            return state.flower.length;
        },

        featherCount: state => {
            return state.feather.length;
        },

        sandCount: state => {
            return state.sand.length;
        },

        cupCount: state => {
            return state.cup.length;
        },

        headCount: state => {
            return state.head.length;
        },

        iterCount: (state) => {
            let count = 1;
            positions.forEach(pos => {
                count *= Math.max(1, state[pos].filter(art => !art.omit).length);
            });

            return count;
        },

        count: state => {
            return count(state);
        },

        valid: (state, getters) => {
            return getters.iterCount < 100000000;
        }
    }
};



export default _store;
