<template>
    <div>
        <div class="up">
            <click-edit-label
                :value="data.name"
                @input="handleChangeKumiName"
                class="title"
            ></click-edit-label>
            <div class="buttons">
                <el-button
                    type="text"
                    icon="el-icon-delete"
                    title="删除"
                    @click="$emit('delete')"
                ></el-button>
                <el-button
                    type="text"
                    icon="el-icon-rank"
                    title="移动"
                    @click="$emit('move')"
                ></el-button>
            </div>
        </div>
        <div class="body">
            <template
                v-for="i in 5"
            >
                <artifact-display
                    :key="i"
                    v-if="isIdValid(data.ids[i - 1])"
                    class="artifact-item"
                    width="200px"
                    :item="getArtifactById(data.ids[i - 1])"
                    selectable
                    @click="$emit('click', i - 1)"
                    @delete="$emit('deleteArtifact', i - 1)"
                    buttons
                    :lock-button="false"
                    :delete-button="true"
                ></artifact-display>
                <add-button
                    :key="i"
                    v-else
                    class="artifact-item"
                    :back="getIcon(i)"
                    @click="$emit('click', i - 1)"
                ></add-button>
            </template>
        </div>
    </div>
</template>

<script>
import { artifactsIcon } from "@asset/artifacts";

import AddButton from "@c/misc/AddButton";
import ArtifactDisplay from "@c/ArtifactDisplay";
import ClickEditLabel from "@c/misc/ClickEditLabel";

export default {
    name: "KumiItem",
    components: {
        AddButton,
        ArtifactDisplay,
        ClickEditLabel,
    },
    props: {
        data: {
            default: () => ({
                name: "test",
                ids: [7001, -1, 2, -3, 4]
            })
        }
    },
    methods: {
        isIdValid(id) {
            if (id < 0) {
                return false;
            }
            let artifactsById = this.$store.getters["artifacts/artifactsById"];
            if (!Object.prototype.hasOwnProperty.call(artifactsById, id)) {
                return false;
            }

            return true;
        },

        getArtifactById(id) {
            let artifactsById = this.$store.getters["artifacts/artifactsById"];
            return artifactsById[id];
        },

        getIcon(i) {
            let temp = [artifactsIcon.flower, artifactsIcon.feather, artifactsIcon.sand, artifactsIcon.cup, artifactsIcon.head];
            return temp[i - 1];
        },

        handleChangeKumiName(text) {
            this.$store.commit("kumi/updateKumiName", {
                id: this.data.id,
                newName: text,
            });
        }
    }
}
</script>

<style lang="scss" scoped>
.up {
    border-bottom: 1px solid #dcdfe6;
    padding: 8px;
    background: #00000005;
    display: flex;
    align-items: center;
    justify-content: space-between;

    .title {
        flex: 1;
        margin-right: 16px;
    }

    .buttons {
        // width: 100px;
        // text-align: right;
        button {
            // margin: 0;
            vertical-align: top;
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