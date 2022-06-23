<template>
    <div>
        <div class="up">
            <p class="title">{{ props.data.title }}</p>
            <div class="buttons">
                <el-button
                    text
                    :icon="IconEpDelete"
                    title="删除"
                    @click="emits('delete')"
                    circle
                ></el-button>
                <el-button
                    text
                    :icon="IconEpEdit"
                    title="重命名"
                    @click="emits('edit')"
                    circle
                ></el-button>
            </div>
        </div>
        <div class="body">
            <template
                v-for="(artifact, index) in artifacts"
                :key="index"
            >
                <artifact-display
                    v-if="artifact"
                    class="artifact-item"
                    width="200px"
                    :item="artifact"
                    selectable
                    @click="emits('click', artifact.position)"
                    @delete="emits('deleteArtifact', artifact.id)"
                    :buttons="true"
                    :lock-button="false"
                    :delete-button="true"
                ></artifact-display>
                <add-button
                    v-else
                    class="artifact-item"
                    @click="emits('click', positions[index])"
                ></add-button>
            </template>
        </div>
    </div>
</template>

<script setup lang="ts">
import { positions } from "@/constants/artifact"

import AddButton from "@c/misc/AddButton"
import ArtifactDisplay from "@c/display/ArtifactDisplay"
import type {KumiItem} from "@/types/kumi"
import type {IArtifact, ArtifactPosition} from "@/types/artifact"
import {useArtifactStore} from "@/store/pinia/artifact"

import IconEpDelete from "~icons/ep/delete"
import IconEpEdit from "~icons/ep/edit"


interface Emits {
    (e: "click", p: ArtifactPosition): void,
    (e: "deleteArtifact", id: number): void,
    (e: "click", p: ArtifactPosition): void,
    (e: "delete"): void,
    (e: "edit"): void
}

const emits = defineEmits<Emits>()

interface Props {
    data: KumiItem
}

const props = defineProps<Props>()


// artifact
const artifactStore = useArtifactStore()

const artifacts = computed(() => {
    let results: (IArtifact | null)[] = []

    const temp: Record<ArtifactPosition, IArtifact> = {}

    for (let id of props.data.artifactIds ?? []) {
        if (id) {
            const artifact = artifactStore.artifacts.value.get(id)
            if (artifact) {
                temp[artifact.position] = artifact
            }
        }
    }

    for (let i = 0; i < 5; i++) {
        const a = temp[positions[i]]
        if (a) {
            results.push(a)
        } else {
            results.push(null)
        }
    }

    return results
})

// export default {
//     computed: {
//         artifactBySlot() {
//             let results = {}
//             for (let artifact of this.artifacts) {
//                 const slot = artifact.position
//                 results[slot] = artifact
//             }
//
//             return results
//         }
//     }
// }
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
            //padding: 0;
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