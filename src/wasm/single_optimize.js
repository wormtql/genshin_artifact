export function wasmSingleOptimize(optimizeConfig, artifacts, timeout = 300000) {
    const worker = new Worker(new URL("@worker/optimize_artifact.js", import.meta.url))

    return new Promise((resolve, reject) => {
        const timer = setTimeout(() => {
            reject("计算超时")
        }, timeout)

        worker.onmessage = e => {
            if (e.data.type === "ready") {
                worker.postMessage({
                    optimizeConfig,
                    artifacts,
                })
            } else {
                const results = e.data.data.results
                clearTimeout(timer)

                resolve(results)
            }
        }

        worker.onerror = () => {
            reject("计算发生错误")
        }
    }).finally(() => {
        worker.terminate()
    })
}
