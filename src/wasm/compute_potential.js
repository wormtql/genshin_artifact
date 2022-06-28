import { deepCopy } from "../utils/common"

export function wasmComputeArtifactPotential(potentialFunctionInterface, artifacts, timeout = 120000) {
    const worker = new Worker(new URL("@worker/compute_potential.worker.js", import.meta.url))
    // console.log(potentialFunctionInterface)

    // because passed value cannot be a proxy
    const a = deepCopy(potentialFunctionInterface)

    return new Promise((resolve, reject) => {
        const timer = setTimeout(() => {
            reject("计算超时")
        }, timeout)

        worker.onmessage = e => {
            if (e.data.type === "ready") {
                worker.postMessage({
                    // potentialFunctionInterface,
                    potentialFunctionInterface: a,
                    // a,
                    artifacts,
                })
            } else {
                const results = e.data.data.results
                clearTimeout(timer)

                resolve(results)
            }
        }

        worker.onerror = e => {
            reject("计算发生错误：" + e.message)
            clearTimeout(timer)
        }
    }).finally(() => {
        worker.terminate()
    })
}
