import Vuex from "vuex";
import Vue from "vue";

const positions = ["flower", "feather", "sand", "cup", "head"];

Vue.use(Vuex);

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
        item.id = id++;
    }
}

let _store = new Vuex.Store({
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

        toggleArtifact(state, obj) {
            let art = state[obj.position][obj.index];
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

        iterCount: (state, getters) => {
            let a = Math.max(getters.flowerCount, 1);
            let b = Math.max(getters.featherCount, 1);
            let c = Math.max(getters.sandCount, 1);
            let d = Math.max(getters.cupCount, 1);
            let e = Math.max(getters.headCount, 1);

            let prod = a * b * c * d * e;
            return prod;
        },

        valid: (state, getters) => {
            return getters.iterCount < 100000000;
        }
    }
})

_store.watch(
    state => ({
        flower: state.flower,
        feather: state.feather,
        sand: state.sand,
        cup: state.cup,
        head: state.head,
    }),
    {
        handler: newValue => {
            localStorage.setItem("artifacts", JSON.stringify(newValue));
        },
        deep: true,
    }
);

export default _store;