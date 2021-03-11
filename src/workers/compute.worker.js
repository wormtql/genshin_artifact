import computeArtifacts from "@alg/attribute_target/compute_artifacts";

const calcs = {
    computeArtifacts,
};

self.addEventListener("message", event => {
    let method = calcs[event.data.method];

    if (method) {
        let args = event.data.args;
        let result = method.apply(null, args);
        self.postMessage({
            message: "done",
            result,
        });
    } else {
        self.postMessage({
            message: "error",
            reason: "no method",
        })
    }

    self.close();
})