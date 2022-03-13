export function wasmComputeArtifactPotential(potentialFunctionInterface, artifacts, timeout = 120000) {
    const worker = new Worker(new URL("@worker/compute_potential.worker.js", import.meta.url))

    return new Promise((resolve, reject) => {
        const timer = setTimeout(() => {
            reject("计算超时")
        }, timeout)

        worker.onmessage = e => {
            if (e.data.type === "ready") {
                worker.postMessage({
                    potentialFunctionInterface,
                    artifacts,
                })
            } else {
                const results = e.data.data.results
                clearTimeout(timer)

                resolve(results)
            }
        }

        // worker.onerror = e => {
        //     reject("计算发生错误：" + e)
        // }
    }).finally(() => {
        worker.terminate()
    })
}
