async function initWasm() {
    const mona = await import("mona")

    self.onmessage = function (e) {
        const input = e.data.input
        const result = mona.TeamOptimizationWasm.optimize_team(input)

        self.postMessage({
            type: "result",
            data: {
                result
            }
        })
    }

    self.postMessage({
        type: "ready"
    })
}

initWasm()
