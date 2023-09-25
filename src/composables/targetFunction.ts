import type {TargetFunctionName} from "@/types/targetFunction"
// @ts-ignore
import {targetFunctionByCharacterName, targetFunctionData} from "@targetFunction"
import {type Ref} from "vue"
import type {CharacterName} from "@/types/character"
import {useI18n} from "@/i18n/i18n";

export function useTargetFunction(characterName: Ref<CharacterName>) {
    const targetFunctionName = ref<TargetFunctionName>("AmberDefault")
    const targetFunctionConfig = ref<any>("NoConfig")
    const targetFunctionUseDSL = ref(false)
    const targetFunctionDSLSource = ref("")

    const { t, ta } = useI18n()

    const targetFunctionBadge = computed(() => {
        return targetFunctionData[targetFunctionName.value].badge
    })

    const targetFunctionDescription = computed(() => {
        // return targetFunctionData[targetFunctionName.value].description
        return ta(targetFunctionData[targetFunctionName.value].description)
    })

    const targetFunctionNeedConfig = computed(() => {
        const temp = targetFunctionData[targetFunctionName.value].config
        return temp && temp.length > 0
    })

    const targetFunctionConfigConfig = computed(() => {
        return targetFunctionData[targetFunctionName.value].config
    })

    const targetFunctionInterface = computed(() => {
        const use_dsl = targetFunctionUseDSL.value
        return {
            name: targetFunctionName.value,
            params: targetFunctionConfig.value,
            use_dsl,
            dsl_source: use_dsl ? targetFunctionDSLSource.value : ""
        }
    })

    watch(() => characterName.value, name => {
        const currentTargetFunctionData = targetFunctionData[targetFunctionName.value]
        if (currentTargetFunctionData["for"] !== "common") {
            // if not common, change to default character specific target function
            const characterTfList = targetFunctionByCharacterName[name]
            if (characterTfList && characterTfList.length > 0) {
                targetFunctionName.value = characterTfList[0].name
            } else {
                targetFunctionName.value = targetFunctionByCharacterName["common"][0].name
            }
        } else {
            // if current is common, change to character default
            const characterTfList = targetFunctionByCharacterName[name]
            if (characterTfList && characterTfList.length > 0) {
                targetFunctionName.value = characterTfList[0].name
            }
        }
    }, {
        flush: "sync"
    })

    watch(() => targetFunctionName.value, name => {
        targetFunctionName.value = name

        const hasConfig = targetFunctionData[name].config.length > 0

        if (hasConfig) {
            let defaultConfig: any = {}
            for (let c of targetFunctionData[name].config) {
                defaultConfig[c.name] = c.default
            }
            targetFunctionConfig.value = {
                [name]: defaultConfig
            }
        } else {
            targetFunctionConfig.value = "NoConfig"
        }
    }, {
        flush: "sync"
    })

    return {
        targetFunctionName,
        targetFunctionConfig,
        targetFunctionUseDSL,
        targetFunctionDSLSource,
        targetFunctionBadge,
        targetFunctionDescription,
        targetFunctionNeedConfig,
        targetFunctionConfigConfig,
        targetFunctionInterface
    }
}

