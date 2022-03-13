export { team_optimize } from "./team_optimize"
export { wasmSingleOptimize } from "./single_optimize"
export { wasmComputeArtifactPotential } from "./compute_potential"

async function initWasm() {
    return await import("mona")
}

export async function wasmBonusPerStat(input) {
    const wasm = await initWasm()
    // console.log(input)

    // console.log(ret)
    return wasm.BonusPerStat.bonus_per_stat(input)
}

export async function wasmGetAttribute(input) {
    const wasm = await initWasm()

    return wasm.CommonInterface.get_attribute(input)
}

export async function wasmGetArtifactsRankByCharacter(characterInterface, weaponInterface, tfInterface, artifacts) {
    const wasm = await initWasm()

    return wasm.CommonInterface.get_artifacts_rank_by_character(
        characterInterface,
        weaponInterface,
        tfInterface,
        artifacts
    )
}
