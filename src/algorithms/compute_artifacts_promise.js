export default function (artifacts, character, weapon, targetFuncName, checkFuncConfig) {
    return new Promise((resolve, reject) => {
        let worker = new Worker("@worker/compute.worker.js", { type: "module" });
        worker.addEventListener("message", event => {
            let data = event.data;
            if (data.message === "error") {
                reject(data.reason);
            } else {
                resolve(data.result);
            }
        });
        worker.postMessage({
            method: "computeArtifacts",
            args: [artifacts, character, weapon, targetFuncName, checkFuncConfig],
        });
    });
}