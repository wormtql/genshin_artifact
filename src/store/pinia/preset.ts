import {upgradePresetItem} from "@/utils/preset"
import {computed, reactive, type Ref, ref, watch} from "vue"
import {type IPreset} from "@/types/preset"

const VERSION = 3

function loadPresetOrDefault(payload: any) {
    if (!payload) {
        return {}
    } else {
        for (let name in payload) {
            let entry = payload[name]
            let item = entry.item

            try {
                entry.item = upgradePresetItem(item)
            } catch (e) {
                console.log("upgrade preset item failed")
                console.log(e)
            }
        }

        return payload
    }
}

export interface PresetEntry {
    name: string,
    item: IPreset,
    version: number
}

function f() {
    const presets: Ref<Record<string, PresetEntry>> = ref(loadPresetOrDefault(null))

    function init(payload: any) {
        presets.value = loadPresetOrDefault(payload)
    }

    function addOrOverwrite(name: string, item: IPreset) {
        presets.value[name] = {
            name,
            item,
            version: VERSION
        }
    }

    function getPreset(name: string): PresetEntry | undefined {
        return presets.value[name]
    }

    function deletePreset(name: string) {
        delete presets.value[name]
    }

    const allFlat = computed(() => {
        return Object.values(presets.value)
    })

    const count = computed(() => {
        return Object.keys(presets.value).length;
    })

    return {
        presets,

        init,
        addOrOverwrite,
        deletePreset,
        getPreset,

        allFlat,
        count
    }
}

const s = f()

export function watchContent() {
    return s.presets.value
}

// watch(() => {
//     return s.presets.value
// }, newValue => {
//     localStorage.setItem("presets5", JSON.stringify(newValue))
// }, {
//     deep: true
// })

export const usePresetStore = () => {
    return s
}
