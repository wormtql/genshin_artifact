async function init_wasm_in_worker() {
    const mona = await import("mona")

    self.onmessage = function (e) {
        const interfac = e.data.interfac
        const results = mona.OptimizeSingleWasm.optimize(interfac)

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
