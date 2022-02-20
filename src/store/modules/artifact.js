import Vue from "vue";

import { count as countArtifacts, hash as hashArtifact } from "@util/artifacts";
import positions from "@const/positions";

// id can only be changed in store mutations
let id = 0;

const hashes = new Map();

function initIdAndHash(state) {
    const { flower, feather, sand, cup, head } = state;
    const allArtifacts = flower.concat(feather).concat(sand).concat(cup).concat(head);
    for (const item of allArtifacts) {
        id = Math.max(id, item.id ?? -1);
        const hash = hashArtifact(item);
        if (hashes.has(hash)) {
            hashes.set(hash, hashes.get(hash) + 1);
        } else {
            hashes.set(hash, 1);
        }
    }
    id++;
    for (const item of allArtifacts) {
        if (!Object.prototype.hasOwnProperty.call(item, "id")) {
            item.id = id++;
        }
    }
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

let _store = {
    namespaced: true,
    state: {
        flower: [],
        feather: [],
        sand: [],
        cup: [],
        head: [],
    },
    mutations: {
        oldInit(state) {
            const localStoredArtifacts = localStorage.getItem("artifacts");
            if (localStoredArtifacts) {
                const obj = JSON.parse(localStoredArtifacts);
                state.flower = obj.flower || [];
                state.feather = obj.feather || [];
                state.sand = obj.sand || [];
                state.cup = obj.cup || [];
                state.head = obj.head || [];
                initIdAndHash(state);
            }
            // localStorage.removeItem("artifacts");
        },

        set(state, payload) {
            payload = payload || {};
            state.flower = payload.flower || [];
            state.feather = payload.feather || [];
            state.sand = payload.sand || [];
            state.cup = payload.cup || [];
            state.head = payload.head || [];
            initIdAndHash(state);
        },

        removeArtifact(state, { position, index }) {
            let art = state[position][index];
            if (art) {
                const hash = hashArtifact(art);
                updateHash(hash, -1);
            }
            state[position].splice(index, 1);
        },

        removeArtifactById(state, { id }) {
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
        },

        addArtifact(state, item) {
            item.id = id++;
            state[item.position].push(item);

            const hash = hashArtifact(item);
            updateHash(hash, 1);
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

        appendArtifacts(state, obj) {
            positions.forEach(pos => {
                for (let art of obj[pos]) {
                    art.id = id++;
                    state[pos].push(art);

                    const hash = hashArtifact(art);
                    updateHash(hash, 1);
                }
            })
        },

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
            return countArtifacts(state);
        },

        valid: (state, getters) => {
            return getters.iterCount < 100000000;
        }
    }
};



export default _store;
