async function init_wasm_in_worker() {
    const mona = await import("mona")

    self.onmessage = function (e) {
        const optimizeConfig = e.data.optimizeConfig
        const artifacts = e.data.artifacts
        const results = mona.OptimizeSingleWasm.optimize(optimizeConfig, artifacts)

        self.postMessage({
            type: "results",
            data: {
                results
            }
        })
    }

    self.postMessage({
        type: "ready"
    })
}

init_wasm_in_worker()
