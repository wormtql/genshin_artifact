import computeArtifactPotential from "@alg/potential/compute_artifact_potential";

self.addEventListener("message", event => {
    let args = event.data.args;

    let type = event.data.type;
    if (type === "single") {
        let result = [args[0], computeArtifactPotential.apply(null, args)];
        self.postMessage({
            message: "done",
            result,
        });
    } else if (type === "all") {
        let result = [];
        let arts = args[0];
        
        for (let art of arts) {
            let value = computeArtifactPotential.call(null, art, ...args.slice(1));
            result.push([art, value]);
        }

        self.postMessage({
            message: "done",
            result,
        });
    }
    

    self.close();
})