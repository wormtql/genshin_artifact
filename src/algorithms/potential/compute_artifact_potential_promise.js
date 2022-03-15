// import Worker from "@worker/compute_potential.worker.js";

function helper(args, type) {
    return new Promise((resolve, reject) => {
        // let worker = new Worker("@worker/compute_potential.worker.js", { type: "module" });
        const worker = new Worker(new URL("@worker/compute_potential.worker.js", import.meta.url))
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
            args,
            type,
        });
    });
}

export function computeSingle(artifact, pfName, pArgs) {
    return helper([artifact, pfName, pArgs], "single");
}

export function computeAll(artifacts, pfName, pArgs) {
    return helper([artifacts, pfName, pArgs], "all");
}