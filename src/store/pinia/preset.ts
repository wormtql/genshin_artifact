import {upgradePresetItem} from "@/utils/preset"
import {computed, reactive, type Ref, ref, watch} from "vue"
import {type IPreset} from "@/types/preset"

const VERSION = 3

function loadLocalOrDefault() {
    const local = localStorage.getItem("presets5")
    if (!local) {
        return {}
    }

    let localObj = null
    try {
        localObj = JSON.parse(local)
    } catch (e) {
        localObj = null
    }

    if (!localObj) {
        return {}
    } else {
        for (let name in localObj) {
            let entry = localObj[name]
            let item = entry.item

            try {
                entry.item = upgradePresetItem(item)
            } catch (e) {
                console.log("upgrade preset item failed")
                console.log(e)
            }
        }

        return localObj
    }
}

interface PresetEntry {
    name: string,
    item: IPreset,
    version: number
}

function f() {
    const presets: Ref<Record<string, PresetEntry>> = ref(loadLocalOrDefault())

    function addOrOverwrite(name: string, item: IPreset) {
        presets.value[name] = {
            name,
            item,
            version: VERSION
        }
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

        addOrOverwrite,
        deletePreset,

        allFlat,
        count
    }
}

const s = f()

watch(() => {
    return s.presets.value
}, newValue => {
    localStorage.setItem("presets5", JSON.stringify(newValue))
}, {
    deep: true
})

export const usePresetStore = () => {
    return s
}