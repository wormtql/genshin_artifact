async function initWasm() {
    const wasm = await import("mona")
    return wasm
}

export async function wasmBonusPerStat(input) {
    const wasm = await initWasm()
    // console.log(input)

    const ret = wasm.BonusPerStat.bonus_per_stat(input)
    // console.log(ret)
    return ret
}
