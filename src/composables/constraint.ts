import type {ArtifactSetName, ArtifactStatName} from "@/types/artifact"
import {convertArtifactName} from "@/utils/converter"

export function useComputeConstraint() {
    const algorithm = ref("AStar")
    const constraintArtifactSet = ref<ArtifactSetName[]>([])
    const constraintSandMainStats = ref<ArtifactStatName[]>([])
    const constraintGobletMainStats = ref<ArtifactStatName[]>([])
    const constraintHeadMainStats = ref<ArtifactStatName[]>([])
    const constraintMinRecharge = ref(1)
    const constraintMinElementalMastery = ref(0)
    const constraintMinCritical = ref(0)
    const constraintMinCriticalDamage = ref(0)

    const constraintSetMode = computed(() => {
        const convertedName = constraintArtifactSet.value.map(x => convertArtifactName(x))
        const len = convertedName.length
        if (len === 2) {
            return {
                "Set22": convertedName
            }
        } else if (len === 1) {
            return {
                "Set4": convertedName[0]
            }
        } else {
            return "Any"
        }
    })

    const constraintInterface = computed(() => {
        let t = {
            "set_mode": constraintSetMode.value,
            "hp_min": null,
            "atk_min": null,
            "def_min": null,
            "recharge_min": constraintMinRecharge.value,
            "em_min": constraintMinElementalMastery.value,
            "crit_min": constraintMinCritical.value,
            "crit_dmg_min": constraintMinCriticalDamage.value
            // "recharge_min": null,
            // "em_min": null,
            // "crit_min": null,
            // "crit_dmg_min": null
        }
        return t
    })

    return {
        algorithm,
        constraintArtifactSet,
        constraintSandMainStats,
        constraintGobletMainStats,
        constraintHeadMainStats,
        constraintMinRecharge,
        constraintMinElementalMastery,
        constraintMinCritical,
        constraintMinCriticalDamage,

        constraintSetMode,
        constraintInterface
    }
}