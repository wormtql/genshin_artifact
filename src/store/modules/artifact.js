import Vue from "vue";

const positions = ["flower", "feather", "sand", "cup", "head"];

// id can only be changed in store mutations
let id = 0;

let flower = [];
let feather = [];
let sand = [];
let cup = [];
let head = [];
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
    }
    id++;
    for (let item of temp) {
        if (Object.prototype.hasOwnProperty.call(item, "id")) {
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
        removeArtifact(state, obj) {
            state[obj.position].splice(obj.index, 1);
        },

        addArtifact(state, item) {
            item.id = id++;
            state[item.position].push(item);
        },

        addArtifactWithID(state, item) {
            state[item.position].push(item);
        },

        toggleArtifact(state, obj) {
            let art = state[obj.position][obj.index];
            art.omit = !art.omit;
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
        setArtifact(state, obj) {
            let n = obj.artifact;
            Vue.set(state[obj.position], obj.index, n);
        },

        appendArtifacts(state, obj) {
            positions.forEach(pos => {
                for (let art of obj[pos]) {
                    art.id = id++;
                    state[pos].push(art);
                }
            })
        },

        removeAllArtifacts(state) {
            state.flower = [];
            state.feather = [];
            state.sand = [];
            state.cup = [];
            state.head = [];
        }
    },
    getters: {
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

        valid: (state, getters) => {
            return getters.iterCount < 100000000;
        }
    }
};



export default _store;