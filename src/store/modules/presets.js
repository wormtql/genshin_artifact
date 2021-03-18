import Vue from "vue";

let temp = localStorage.getItem("presets");

let presets = temp ? JSON.parse(temp) : {};

let item = {
    namespaced: true,
    state: {
        presets,
    },
    mutations: {
        add(state, payload) {
            if (!Object.prototype.hasOwnProperty.call(state.presets, payload.name)) {
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