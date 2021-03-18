import Vuex from "vuex";
import Vue from "vue";

import artifacts from "./modules/artifact";
import presets from "./modules/presets";

Vue.use(Vuex);

const _store = new Vuex.Store({
    modules: {
        artifacts,
        presets,
    }
});

_store.watch(
    state => ({
        flower: state.artifacts.flower,
        feather: state.artifacts.feather,
        sand: state.artifacts.sand,
        cup: state.artifacts.cup,
        head: state.artifacts.head,
    }),
    newValue => {
        localStorage.setItem("artifacts", JSON.stringify(newValue));
    },
    {
        deep: true,
    },
);

_store.watch(
    state => state.presets.presets,
    newValue => {
        localStorage.setItem("presets", JSON.stringify(newValue));
    },
    {
        deep: true,
    }
)

export default _store;