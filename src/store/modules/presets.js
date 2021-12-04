import Vue from "vue";

import upgradePreset from "../utils/upgradePreset";

const VERSION_PRESET = "2";
const VERSION_PRESET_INT = parseInt(VERSION_PRESET);

let temp = localStorage.getItem("presets");

// presets form localStorage, might be out-versioned
let presets = temp ? JSON.parse(temp) : {};

let upgradedPresets = {};
let upgradedCount = 0;
let invalidCount = 0;

// upgrade lower version presets
for (let name in presets) {
    if (!name) {
        continue;
    }
    let preset = presets[name];
    if (!preset) {
        continue;
    }

    let version = parseInt(preset.version ?? "1");
    if (version < VERSION_PRESET_INT) {
        // need upgrade

        try {
            let newPreset = upgradePreset(preset);
            newPreset.version = VERSION_PRESET;
            upgradedPresets[newPreset.name] = newPreset;
            upgradedCount++;
        } catch (e) {
            console.error(e);
            invalidCount++;
        }
    } else {
        // don't need upgrade
        console.log("not upgrade");
        upgradedPresets[preset.name] = preset;
    }
}

if (invalidCount > 0 || upgradedCount > 0) {
    Vue.nextTick(() => {
        window.monaApp.message(`已升级预设，通过${upgradedCount}个，失败${invalidCount}个`);
    });
}
if (upgradedCount > 0) {
    localStorage.setItem("presets", JSON.stringify(upgradedPresets));
}


let item = {
    namespaced: true,
    state: {
        presets: upgradedPresets,
    },
    mutations: {
        add(state, payload) {
            if (!Object.prototype.hasOwnProperty.call(state.presets, payload.name)) {
                let preset = payload.value;
                Vue.set(preset, "version", VERSION_PRESET);
                Vue.set(state.presets, payload.name, payload.value);
            }
        },

        overwrite(state, { name, preset }) {
            Vue.set(preset, "version", VERSION_PRESET);
            Vue.set(state.presets, name, preset);
        },

        update(state, preset) {
            let name = preset.name;
            Vue.set(preset, "version", VERSION_PRESET);
            Vue.set(state.presets, name, preset);
        },

        delete(state, payload) {
            let name = payload.name;
            Vue.delete(state.presets, name);
        }
    },
    getters: {
        all(state) {
            return state.presets;
        },

        count(state) {
            return Object.keys(state.presets).length;
        },

        // firstName(state, getters) {
        //     if (getters.count === 0) {
        //         return "";
        //     }
        //     return Object.keys(state.presets)[0];
        // }
    }
}

export default item;