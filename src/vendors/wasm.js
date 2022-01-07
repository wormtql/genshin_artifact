import Vue from "vue"

async function initWasm() {
    const mona = await import("mona");
    Vue.prototype.$mona = mona;
}

initWasm();