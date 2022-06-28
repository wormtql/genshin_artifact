async function initMonaWasm() {
    const mona = await import("../../mona_wasm/pkg")
    // const { memory } = await import("../../mona_wasm/pkg/mona_wasm_bg.wasm")

    return mona
}

const mona = initMonaWasm()

export async function useMona() {
    return await mona
}