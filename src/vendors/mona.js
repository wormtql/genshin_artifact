import Vue from "vue"

async function init() {
    const mona = await import("mona")
    const { memory } = await import("mona/mona_bg.wasm")

    mona.memory = memory
    Vue.prototype.$mona = mona


    // Vue.prototype.$character = await getAsset("character")
    // // console.log(Vue.prototype.$character)
    // Vue.prototype.$weapon = await getAsset("weapon")
    // Vue.prototype.$buff = await getAsset("buff")
    // Vue.prototype.$artifact = await getAsset("artifact")
    // Vue.prototype.$targetFunction = await getAsset("targetFunction")
}

const monaPromise = init()

export function requestMonaWasm() {
    return monaPromise
}
