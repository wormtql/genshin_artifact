import { IArtifactWasm } from "@/types/artifact"

export function wasmSingleOptimize(optimizeConfig: object, artifacts: IArtifactWasm[], timeout?: number, needCancel?: false): Promise<any>
export function wasmSingleOptimize(optimizeConfig: object, artifacts: IArtifactWasm[], timeout: number, needCancel: true): [Promise<any>, () => void]
export function wasmSingleOptimize(optimizeConfig: object, artifacts: IArtifactWasm[], timeout = 120000, needCancel = false): Promise<any> | [Promise<any>, () => void]{
    const worker = new Worker(new URL("@worker/optimize_artifact.js", import.meta.url))
    let cancel: (() => void) | null = null
    let timer: number = -1
    const promise = new Promise((resolve, reject) => {
        cancel = () => {
            reject("计算被中止")
        }

        timer = setTimeout(() => {
            reject("计算超时")
        }, timeout) as unknown as number

        worker.onmessage = e => {
            if (e.data.type === "ready") {
                worker.postMessage({
                    optimizeConfig,
                    artifacts,
                })
            } else {
                const results = e.data.data.results
                resolve(results)
            }
        }

        worker.onerror = () => {
            reject("计算发生错误")
        }
    }).finally(() => {
        worker.terminate()
        clearTimeout(timer)
    })

    return needCancel ? [promise, cancel!] : promise
}
