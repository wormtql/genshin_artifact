import type {WeaponType} from "@/types/weapon"
// @ts-ignore
import { characterData } from "@character"
import {type Ref} from "vue"
import type {CharacterName} from "@/types/character"
import {useI18n} from "@/i18n/i18n"

export function useCharacter() {
    const characterName = ref("Amber")
    const characterLevel = ref("90")
    const characterConfig = ref<any>("NoConfig")
    const characterSkill1 = ref(8)
    const characterSkill2 = ref(8)
    const characterSkill3 = ref(8)
    const characterConstellation = ref(0)

    const { ta } = useI18n()
    // const characterWeaponType = ref<WeaponType>("Bow")

    const characterWeaponType = computed(() => {
        const data = characterData[characterName.value]
        return data.weapon
    })

    const characterLevelNumber = computed((): number => {
        return parseInt(characterLevel.value)
    })

    const characterAscend = computed((): boolean => {
        return characterLevel.value.includes("+")
    })

    const characterSplash = computed((): string => {
        const data = characterData[characterName.value]
        return data.splash
    })

    const characterNeedConfig = computed(() => {
        let temp = characterData[characterName.value].config
        return temp && temp.length > 0
    })

    const characterConfigConfig = computed(() => {
        return characterData[characterName.value].config
    })

    const characterLocale = computed(() => {
        return ta(characterData[characterName.value].nameLocale)
    })

    const characterInterface = computed(() => {
        let i = {
            name: characterName.value,
            level: characterLevelNumber.value,
            ascend: characterAscend.value,
            constellation: characterConstellation.value,
            skill1: characterSkill1.value - 1,
            skill2: characterSkill2.value - 1,
            skill3: characterSkill3.value - 1,
            params: characterConfig.value
        }
        return i
    })

    watch(() => characterName.value, name => {
        characterName.value = name

        const hasConfigData = characterData[name].config.length > 0;

        // change config
        if (hasConfigData) {
            const configs = characterData[name].config

            let defaultConfig: any = {}
            for (let c of configs) {
                defaultConfig[c.name] = c.default
            }
            characterConfig.value = {
                [name]: defaultConfig
            }
        } else {
            characterConfig.value = "NoConfig"
        }
    }, {
        flush: "sync"
    })

    return {
        characterName,
        characterLevel,
        characterConfig,
        characterSkill1,
        characterSkill2,
        characterSkill3,
        characterConstellation,
        characterWeaponType,
        characterLevelNumber,
        characterAscend,
        characterSplash,
        characterNeedConfig,
        characterConfigConfig,
        characterInterface,
        characterLocale,
    }
}

export function useCharacterSkill(characterName: Ref<CharacterName>) {
    const characterSkillConfig = ref<any>("NoConfig")
    const characterSkillIndex = ref(0)

    const characterNeedSkillConfig = computed((): boolean => {
        let temp = characterData[characterName.value].configSkill
        return temp && temp.length > 0
    })

    const characterSkillConfigConfig = computed(() => {
        return characterData[characterName.value].configSkill
    })

    const characterSkillInterface = computed(() => {
        return {
            index: characterSkillIndex.value,
            config: characterSkillConfig.value
        }
    })

    watch(() => characterName.value, name => {
        const hasConfigSkill = characterData[name].configSkill.length > 0

        // change skill config
        if (hasConfigSkill) {
            let defaultConfig: any = {}
            for (let c of characterData[name].configSkill) {
                defaultConfig[c.name] = c.default
            }
            characterSkillConfig.value = {
                [name]: defaultConfig
            }
        } else {
            characterSkillConfig.value = "NoConfig"
        }

        // change skill index
        characterSkillIndex.value = 0
    }, {
        flush: "sync"
    })

    // watch(() => characterSkillConfig.value, () => {
    //     console.log("skill config")
    // })

    // watch(() => characterSkillIndex.value, v => {
    //     console.log(v)
    // })

    return {
        characterSkillConfig,
        characterSkillIndex,
        characterNeedSkillConfig,
        characterSkillConfigConfig,
        characterSkillInterface,
    }
}
