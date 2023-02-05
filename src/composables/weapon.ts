import type {WeaponName, WeaponType} from "@/types/weapon"
// @ts-ignore
import {weaponByType, weaponData} from "@weapon"
import {type Ref} from "vue"
import {useI18n} from "@/i18n/i18n";

export function useWeapon(weaponType: null | Ref<WeaponType>) {
    const weaponName = ref<WeaponName>("PolarStar")
    const weaponLevel = ref("90")
    const weaponRefine = ref(1)
    const weaponConfig = ref<any>({
        "PolarStar": {
            stack: 1
        }
    })

    const weaponLevelNumber = computed(() => {
        return parseInt(weaponLevel.value)
    })

    const weaponAscend = computed(() => {
        return weaponLevel.value.includes("+")
    })

    const weaponSplash = computed(() => {
        const data = weaponData[weaponName.value]
        return data.gacha ?? data.url ?? data.tn
    })

    const weaponNeedConfig = computed(() => {
        return !!weaponData[weaponName.value].configs
    })

    const weaponConfigConfig = computed(() => {
        return weaponData[weaponName.value].configs
    })

    const weaponInterface = computed(() => {
        return {
            name: weaponName.value,
            level: weaponLevelNumber.value,
            ascend: weaponAscend.value,
            refine: weaponRefine.value,
            params: weaponConfig.value
        }
    })

    // function changeWeapon(name: WeaponName) {
    //
    // }

    if (weaponType) {
        watch(() => weaponType.value, newWeaponType => {
            const defaultWeaponData = weaponByType[newWeaponType][0]
            weaponName.value = defaultWeaponData.name
        }, {
            flush: "sync"
        })
    }

    watch(() => weaponName.value, name => {
        weaponName.value = name

        // change config
        const hasConfig = !!weaponData[name]?.configs
        if (hasConfig) {
            const configs = weaponData[name].configs

            let defaultConfig: any = {}
            for (let config of configs) {
                defaultConfig[config.name] = config.default
            }

            weaponConfig.value = {
                [name]: defaultConfig
            }
        } else {
            weaponConfig.value = "NoConfig"
        }
    }, {
        flush: "sync"
    })

    const { ta } = useI18n()

    const weaponLocale = computed(() => {
        return ta(weaponData[weaponName.value].nameLocale)
    })

    return {
        weaponName,
        weaponLevel,
        weaponRefine,
        weaponConfig,
        weaponLevelNumber,
        weaponAscend,
        weaponSplash,
        weaponNeedConfig,
        weaponConfigConfig,
        weaponInterface,
        weaponLocale

        // changeWeapon
    }
}