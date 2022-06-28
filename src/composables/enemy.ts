export function useEnemy() {
    const enemyConfig = ref({
        level: 90,
        electro_res: 0.1,
        pyro_res: 0.1,
        hydro_res: 0.1,
        cryo_res: 0.1,
        geo_res: 0.1,
        anemo_res: 0.1,
        dendro_res: 0.1,
        physical_res: 0.1
    })

    const enemyInterface = computed(() => {
        return enemyConfig.value
    })

    return {
        enemyConfig,
        enemyInterface
    }
}