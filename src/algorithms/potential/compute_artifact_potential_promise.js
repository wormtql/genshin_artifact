// import Worker from "@worker/compute_potential.worker.js";

export function computeSingle(artifact, pfName, pArgs) {
    return new Promise((resolve, reject) => {
        let worker = new Worker("@worker/compute_potential.worker.js", { type: "module" });
        // let worker = new Worker();
        worker.addEventListener("message", event => {
            let data = event.data;
            if (data.message === "error") {
                reject(data.reason);
            } else {
                resolve(data.result);
            }
        });
        worker.postMessage({
            args: [artifact, pfName, pArgs],
            type: "single",
        });
    });
}

export function computeAll(artifacts, pfName, pArgs) {
    return new Promise((resolve, reject) => {
        let worker = new Worker("@worker/compute_potential.worker.js", { type: "module" });
        // let worker = new Worker();
        worker.addEventListener("message", event => {
            let data = event.data;
            if (data.message === "error") {
                reject(data.reason);
            } else {
                resolve(data.result);
            }
        });
        worker.postMessage({
            args: [artifacts, pfName, pArgs],
            type: "all",
        });
    });
}