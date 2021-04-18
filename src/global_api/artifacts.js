import store from "@/store/store";

import positions from "@const/positions";

import checkImportJson from "@util/checkImportJson";

export function updateAllArtifacts(artifacts) {
    let { artifacts: checkedArtifacts } = checkImportJson(artifacts);

    store.commit("artifacts/removeAllArtifacts");

    for (let pos of positions) {
        for (let art of checkedArtifacts[pos]) {
            store.commit("artifacts/addArtifact", art);
        }
    }
}

// this function suppose you have ID field on you artifact, it won't do any check
export function updateAllArtifactsWithID(artifacts) {
    let { artifacts: checkedArtifacts } = checkImportJson(artifacts);

    store.commit("artifacts/removeAllArtifacts");

    for (let pos of positions) {
        for (let art of checkedArtifacts[pos]) {
            store.commit("artifacts/addArtifactWithID", art);
        }
    }
}

export function getAllArtifacts() {
    return store.getters["artifacts/allArtifacts"];
}