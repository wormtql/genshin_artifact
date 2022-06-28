// @ts-ignore
import {buffData} from "@buff"
import {RandomIDProvider} from "@/utils/idProvider"
import type {IBuffWasm} from "@/types/preset"

export interface BuffEntry {
    id: number,
    name: string,
    config: any,
    lock: boolean,
}

export function useBuff() {
    const buffs = ref<BuffEntry[]>([])

    const idGenerator = new RandomIDProvider()

    const buffsUnlocked = computed((): BuffEntry[] => {
        return buffs.value.filter(e => !e.lock)
    })

    const buffsInterface = computed((): IBuffWasm[] => {
        let temp = []
        for (let buff of buffsUnlocked.value) {
            temp.push({
                name: buff.name,
                config: buff.config
            })
        }
        return temp
    })

    function addBuff(name: string) {
        const data = buffData[name]
        let defaultConfig: any = {}
        for (let c of data.config) {
            defaultConfig[c.name] = c.default
        }

        let config
        if (data.config.length === 0) {
            config = "NoConfig"
        } else {
            config = {
                [name]: defaultConfig
            }
        }

        buffs.value.push({
            name,
            config,
            id: idGenerator.generateId(),
            lock: false
        })
    }

    function deleteBuff(id: number) {
        const index = buffs.value.findIndex(e => e.id === id)
        buffs.value.splice(index, 1)
    }

    function toggleBuff(id: number) {
        const index = buffs.value.findIndex(e => e.id === id)
        const v = buffs.value[index].lock
        buffs.value[index].lock = !v
    }

    return {
        buffs,

        buffsInterface,

        addBuff,
        deleteBuff,
        toggleBuff,
    }
}

