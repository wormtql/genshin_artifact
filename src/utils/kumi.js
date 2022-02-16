import store from "@/store/store"

export function getNextDirName() {
    let existingNames = store.getters["kumi/dirNames"]

    for (let i = 0; i < 100; i++) {
        const tempName = `收藏夹${i}`
        if (!existingNames.has(tempName)) {
            return tempName
        }
    }

    return `收藏夹${Math.floor(Math.random() * 1e5)}`
}

export function getKumiNames(dirId) {
    const kumis = getKumisByDir(dirId)
    let set = new Set()
    for (let kumi of kumis) {
        set.add(kumi.title)
    }

    return set
}

export function getNextKumiName(dirId) {
    let set = getKumiNames(dirId)

    for (let i = 0; i < 100; i++) {
        const tempName = `圣遗物组${i}`
        if (!set.has(tempName)) {
            return tempName
        }
    }

    return `圣遗物组${Math.floor(Math.random() * 1e5)}`
}

export function newDir(name) {
    const finalName = name ?? getNextDirName()
    store.commit("kumi/newDir", { name: finalName })
}

export function checkName(name) {
    if (name === "" || !name) {
        return {
            error: true,
            reason: "名称为空"
        }
    }

    const existingName = store.getters['kumi/dirNames']
    if (existingName.has(name)) {
        return {
            error: true,
            reason: "名称已存在"
        }
    }

    return {
        error: false
    }
}

export function getKumisByDir(dirId) {
    const ids = store.getters["kumi/kumiByDir"][dirId]
    // console.log("ids", ids)

    if (!ids) {
        return []
    }

    const kumi = store.state.kumi.kumi

    let results = []
    for (let id of ids) {
        results.push(kumi[id])
    }

    return results
}

export function checkKumiName(dirId, name) {
    if (name === "" || !name) {
        return {
            error: true,
            reason: "名称为空"
        }
    }

    let set = getKumiNames(dirId)
    if (set.has(name)) {
        return {
            error: true,
            reason: "名称已存在"
        }
    }

    return {
        error: false
    }
}

export function newKumi(dirId, name) {
    const finalName = name ?? getNextKumiName(dirId)

    store.commit("kumi/newKumi", {
        artifactIds: [],
        title: finalName,
        dirId
    })
}

export function newKumiWithArtifacts(dirId, name, artifactIds) {
    store.commit("kumi/newKumi", {
        artifactIds,
        title: name,
        dirId
    })
}

export function deleteKumi(id) {
    store.commit("kumi/deleteKumi", { id })
}

export function renameItem(id, name) {
    store.commit("kumi/renameItem", { id, name })
}

export function deleteDir(id) {
    store.commit("kumi/deleteDir", { id })
}

export function getItem(id) {
    return store.state.kumi.kumi[id]
}

export function updateKumiArtifact(kumiId, artifactId) {
    const artifactsById = store.getters["artifacts/artifactsById"]

    const artifact = artifactsById[artifactId]
    if (!artifact) {
        return
    }

    const position = artifact.position
    let ids = [artifactId]

    const item = getItem(kumiId)
    if (!item) {
        return
    }

    const originalIds = item.artifactIds ?? []
    for (let id of originalIds) {
        const art = artifactsById[id]
        if (art && art.position === position) {
            continue
        }

        ids.push(id)
    }

    store.commit("kumi/updateKumiArtifact", {
        artifactIds: ids,
        kumiId
    })
}

export function removeKumiArtifact(kumiId, removeId) {
    const item = getItem(kumiId)
    if (!item) {
        return
    }

    let ids = item.artifactIds ?? []
    // console.log(ids)
    let index = ids.indexOf(removeId)
    if (index !== -1) {
        ids.splice(index, 1)
    }
    // console.log(ids)

    store.commit("kumi/updateKumiArtifact", {
        artifactIds: ids,
        kumiId
    })
}

export function getArtifactIdsByKumiId(kumiId) {
    const item = getItem(kumiId)
    if (!item) {
        return []
    }

    let ids = item.artifactIds ?? []
    let set = new Set()
    for (let id of ids) {
        set.add(id)
    }

    return set
}
