import Vue from "vue"

async function initWasm() {
    const mona = await import("mona")
    const { memory } = await import("mona/mona_bg.wasm")

    mona.memory = memory
    Vue.prototype.$mona = mona
    // Vue.prototype
}

window.wasmPromise = initWasm()