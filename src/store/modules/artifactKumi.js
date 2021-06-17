import Vue from "vue";

import getTreeNode from "../utils/getTreeNode";

let id = 2;


let tree = {
    label: "root",
    type: "dir",
    id: 0,
    children: [
        {
            label: "默认收藏夹",
            type: "dir",
            id: 1,
        }
    ],
};

let temp = localStorage.getItem("kumiTree");
if (temp) {
    tree = JSON.parse(temp).tree;
    let queue = [tree];
    while (queue.length > 0) {
        let p = queue.pop();
        if (Object.prototype.hasOwnProperty.call(p, "id")) {
            id = Math.max(p.id, id);
        }
        if (p.type === "dir") {
            for (let child of p.children ?? []) {
                queue.push(child);
            }
        }
    }
    id++;
    console.log("next kumi node ID: " + id);
}

function getKumiFromTree(node) {
    let temp = [];
    for (let child of node.children ?? []) {
        if (child.type === "data") {
            temp.push(child.data);
        }
    }

    return temp;
}


export default {
    namespaced: true,
    state: {
        // kumi: {},
        tree,
    },
    mutations: {
        // create artifacts group
        newKumi(state, { ids, label, under }) {
            let node = getTreeNode(state.tree, node => node.id === under);
            if (!node || node.type !== "dir") {
                return;
            }
            if (!node.children) {
                Vue.set(node, "children", []);
            }
            let newId = id++;
            node.children.push({
                type: "data",
                label,
                id: newId,
                data: {
                    name: label,
                    ids,
                    id: newId,
                }
            })
        },

        // delete artifacts group
        deleteKumi(state, { id }) {
            let queue = [state.tree];
            a: while (queue.length > 0) {
                let p = queue.pop();

                let children = p.children ?? [];
                for (let i = 0; i < children.length; i++) {
                    let child = children[i];
                    if (child.type === "data" && child.id === id) {
                        children.splice(i, 1);
                        break a;
                    }
                    if (child.type === "dir") {
                        queue.push(child);
                    }
                }
            }
        },

        // update directory name
        updateDirName(state, { id, newName }) {
            let node = getTreeNode(state.tree, node => node.id === id);
            if (!node) {
                return;
            }
            node.label = newName;
        },

        // update artifact in a group
        updateKumiArtifact(state, { id, posId, newId }) {
            let node = getTreeNode(state.tree, node => node.id === id);
            if (!node) {
                return;
            }

            node.data = node.data ?? { ids: [-1, -1, -1, -1, -1] }
            Vue.set(node.data.ids, posId, newId);
            // node.data.ids[posId] = newId;
        },

        // update group name
        updateKumiName(state, { id, newName }) {
            let node = getTreeNode(state.tree, node => node.id === id);
            if (!node || node.type !== "data") {
                return;
            }

            node.label = newName;
            node.data = node.data ?? {};
            node.data.name = newName;
        },

        // create new directory
        newDir(state, { name }) {
            let root = state.tree;
            root.children.push({
                label: name,
                type: "dir",
                id: id++,
            });
        },

        // delete directory
        deleteDir(state, { id }) {
            let queue = [state.tree];
            a: while (queue.length > 0) {
                let p = queue.pop();

                let children = p.children ?? [];
                for (let i = 0; i < children.length; i++) {
                    let child = children[i];
                    if (child.type === "dir" && child.id === id) {
                        children.splice(i, 1);
                        break a;
                    }
                    if (child.type === "dir") {
                        queue.push(child);
                    }
                }
            }
        }
    },
    getters: {
        kumiByDir: state => {
            let temp = {};
            
            for (let child of state.tree.children) {
                temp[child.id] = getKumiFromTree(child);
            }

            return temp;
        },

        firstDirId: state => {
            let temp = state.tree.children[0].id;
            return temp ?? -1;
        }
    }
}