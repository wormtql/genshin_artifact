<template>
    <div>
        <div class="up">
            <p class="title">{{ data.title }}</p>
            <div class="buttons">
                <el-button
                    type="text"
                    icon="el-icon-delete"
                    title="删除"
                    @click="$emit('delete')"
                ></el-button>
                <el-button
                    type="text"
                    icon="el-icon-edit"
                    title="重命名"
                    @click="$emit('edit')"
                ></el-button>
            </div>
        </div>
        <div class="body">
            <template
                v-for="pos in positions"
            >
                <artifact-display
                    :key="pos"
                    v-if="artifactBySlot[pos]"
                    class="artifact-item"
                    width="200px"
                    :item="artifactBySlot[pos]"
                    selectable
                    @click="$emit('click', pos)"
                    @delete="$emit('deleteArtifact', artifactBySlot[pos].id)"
                    buttons
                    :lock-button="false"
                    :delete-button="true"
                ></artifact-display>
                <add-button
                    :key="pos"
                    v-else
                    class="artifact-item"
                    @click="$emit('click', pos)"
                ></add-button>
            </template>
        </div>
    </div>
</template>

<script>
// import { artifactsIcon } from "@asset/artifacts"
import { mapGetters } from "vuex"
import { positions } from "@const/artifact"

import AddButton from "@c/misc/AddButton"
import ArtifactDisplay from "@c/ArtifactDisplay"
// import SelectArtifact from "@c/select/SelectArtifact"

export default {
    name: "KumiItem",
    components: {
        AddButton,
        ArtifactDisplay,
        // SelectArtifact,
    },
    props: ["data"],
    created() {
        this.positions = positions
    },
    computed: {
        ...mapGetters("artifacts", [
            "artifactsById"
        ]),

        artifacts() {
            let results = []
            for (let id of this.data.artifactIds) {
                if (Object.prototype.hasOwnProperty.call(this.artifactsById, id)) {
                    results.push(this.artifactsById[id])
                }
            }
            return results
        },

        artifactBySlot() {
            let results = {}
            for (let artifact of this.artifacts) {
                const slot = artifact.position
                results[slot] = artifact
            }

            return results
        }
    }
}
</script>

<style lang="scss" scoped>
.up {
    border-bottom: 1px solid #dcdfe6;
    padding: 8px;
    //background: #00000005;
    display: flex;
    align-items: center;
    justify-content: space-between;

    .title {
        //flex: 1;
        //margin-right: 16px;
        color: #606266;
        margin: 0;
    }

    .buttons {
        // width: 100px;
        // text-align: right;
        button {
            // margin: 0;
            vertical-align: top;
            padding: 0;
        }
    }
}

.body {
    // display: flex;
    // overflow: auto;
    overflow-x: auto;
    white-space: nowrap;
    padding: 16px;

    .artifact-item {
        margin-right: 16px;
        vertical-align: middle;
        width: 200px;
        height: 130px;

        &:last-of-type {
            margin-right: 0;
        }
    }
}
</style>