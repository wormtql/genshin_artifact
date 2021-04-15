import store from "@/store/store";

import positions from "@const/positions";

import checkImportJson from "@util/checkImportJson";

export function updateAllArtifacts(artifacts) {
    let { artifacts: checkedArtifacts } = checkImportJson(artifacts);
    console.log(checkedArtifacts);

    store.commit("artifacts/removeAllArtifacts");

    for (let pos of positions) {
        for (let art of checkedArtifacts[pos]) {
            store.commit("artifacts/addArtifact", art);
        }
    }
}