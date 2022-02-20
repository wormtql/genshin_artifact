import Vue from "vue"

// let ID = 0

// let tree = {
//     label: "root",
//     type: "dir",
//     id: 0,
//     children: [
//         {
//             label: "默认收藏夹",
//             type: "dir",
//             id: 1,
//         }
//     ],
// };
//
// let temp = localStorage.getItem("kumiTree");
// if (temp) {
//     tree = JSON.parse(temp).tree;
//     let queue = [tree];
//     while (queue.length > 0) {
//         let p = queue.pop();
//         if (Object.prototype.hasOwnProperty.call(p, "id")) {
//             id = Math.max(p.id, id);
//         }
//         if (p.type === "dir") {
//             for (let child of p.children ?? []) {
//                 queue.push(child);
//             }
//         }
//     }
//     id++;
//     // console.log("next kumi node ID: " + id);
// }
//
// function getKumiFromTree(node) {
//     let temp = [];
//     for (let child of node.children ?? []) {
//         if (child.type === "data") {
//             temp.push(child.data);
//         }
//     }
//
//     return temp;
// }

function nextID() {
    return Math.floor(Math.random() * 1e9)
}

function isDir(item) {
    return item.dir
}

function getDefault() {
    return {
        0: {
            id: 0,
            title: "默认收藏夹",
            dir: true,
            children: []
        }
    }
}

function newDir(id, title) {
    return {
        id,
        title,
        dir: true,
        children: []
    }
}

function newKumi(id, title, artifactIds, dirId) {
    return {
        id,
        title,
        dir: false,
        artifactIds,
        parent: dirId
    }
}

export default {
    namespaced: true,
    state: {
        kumi: getDefault(),
    },
    mutations: {
        set(state, payload) {
            if (payload) {
                state.kumi = payload.kumi
            }
        },

        newKumi(state, { artifactIds, title, dirId }) {
            const id = nextID()
            let kumi = newKumi(id, title, artifactIds, dirId)

            let dirItem = state.kumi[dirId]
            dirItem.children.push(id)

            Vue.set(state.kumi, id, kumi)
        },

        deleteKumi(state, { id }) {
            const item = state.kumi[id]
            const dirId = item.parent

            Vue.delete(state.kumi, id)
            const dirItem = state.kumi[dirId]
            const index = dirItem.children.indexOf(id)
            if (index !== -1) {
                Vue.delete(dirItem.children, index)
            }
        },

        updateKumiArtifact(state, { artifactIds, kumiId }) {
            Vue.set(state.kumi[kumiId], "artifactIds", artifactIds)
        },

        renameItem(state, { id, name }) {
            let item = state.kumi[id]
            if (!item) {
                return
            }
            Vue.set(item, "title", name)
        },

        // create new directory
        newDir(state, { name }) {
            const id = nextID()

            Vue.set(state.kumi, id, newDir(id, name))
        },

        deleteDir(state, { id }) {
            const item = state.kumi[id]
            if (!item) {
                return
            }

            for (let childId of item.children) {
                Vue.delete(state.kumi, childId)
            }
            Vue.delete(state.kumi, id)
        }
    },
    getters: {
        directories: (state, getters) => {
            let results = []
            for (let id of getters.idsAscend) {
                const item = state.kumi[id]
                if (isDir(item)) {
                    results.push({
                        title: item.title,
                        id: item.id
                    })
                }
            }
            return results
        },

        kumiByDir(state, getters) {
            let results = {}
            for (let dir of getters.directories) {
                const item = state.kumi[dir.id]
                results[item.id] = item.children
            }
            return results
        },

        idsAscend: state => {
            let temp = Object.keys(state.kumi)
            temp.sort((a, b) => parseInt(a) - parseInt(b))
            // console.log("ids ascend", temp)
            return temp
        },

        kumis: (state, getters) => {
            let results = []
            for (let id of getters.idsAscend) {
                const item = state.kumi[id]
                if (!isDir(item)) {
                    results.push({
                        title: item.title,
                        id: item.id
                    })
                }
            }

            return results
        },

        kumiNames: (state, getters) => {
            let set = new Set()
            for (let item of getters.kumis) {
                set.add(item.title)
            }

            return set
        },

        dirNames(state, getters) {
            let set = new Set()
            for (let item of getters.directories) {
                set.add(item.title)
            }
            return set
        },

        treeDataForElementUI: (state, getters) => {
            function helper(id) {
                const item = state.kumi[id]
                if (!isDir(item)) {
                    return {
                        label: item.title,
                        kumiId: id
                    }
                }

                let children = []
                for (let i of item.children) {
                    children.push(helper(i))
                }

                return {
                    label: item.title,
                    children
                }
            }

            let data = []
            for (let dir of getters.directories) {
                data.push(helper(dir.id))
            }

            return data
        }
    }
}
