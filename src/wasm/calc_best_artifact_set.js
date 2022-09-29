import { deepCopy } from "@/utils/common"

export function wasmCalcBestArtifactSet(calcInterface, timeout = 120000) {
    const worker = new Worker(new URL("@worker/general_compute.worker.js", import.meta.url))

    const a = deepCopy(calcInterface)

    return new Promise((resolve, reject) => {
        const timer = setTimeout(() => {
            reject("计算超时")
        }, timeout)

        worker.onmessage = e => {
            if (e.data.type === "ready") {
                worker.postMessage({
                    args: [a],
                    dispatch: "best_artifact_set"
                })
            } else if (e.data.type === "result") {
                const result = e.data.result
                clearTimeout(timer)

                resolve(result)
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
