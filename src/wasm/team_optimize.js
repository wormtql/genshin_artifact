export function team_optimize(input, artifacts) {
    const worker = new Worker(new URL("@worker/team_optimization.js", import.meta.url))

    return new Promise((resolve, reject) => {
        worker.onmessage = function (e) {
            const type = e.data.type
            if (type === "ready") {
                worker.postMessage({
                    input,
                    artifacts
                })
            } else if (type === "result") {
                const result = e.data.data.result

                worker.terminate()
                resolve(result)
            }
        }

        worker.onerror = function (e) {
            reject(e)
        }
    })
}