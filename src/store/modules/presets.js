import Vue from "vue";

const VERSION_PRESET = "2";

let temp = localStorage.getItem("presets");

let presets = temp ? JSON.parse(temp) : {};

// upgrade lower version presets
// for (let preset of Object.keys(presets)) {
//     if ()
// }


let item = {
    namespaced: true,
    state: {
        presets,
    },
    mutations: {
        add(state, payload) {
            if (!Object.prototype.hasOwnProperty.call(state.presets, payload.name)) {
                let preset = payload.value;
                preset.version = VERSION_PRESET;
                Vue.set(state.presets, payload.name, payload.value);
            }
        },

        delete(state, payload) {
            let name = payload.name;
            Vue.delete(state.presets, name);
        }
    },
    getters: {
        all(state) {
            return state.presets;
        }
    }
}

export default item;