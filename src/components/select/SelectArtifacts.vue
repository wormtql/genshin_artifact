<template>
    <div>
        <el-tabs
            v-model="activePosition"
        >
            <el-tab-pane
                v-for="position in positions"
                :key="position"
                :name="position"
                :label="positionToLocale(position)"
                class="artifact-panel"
            >
                <artifact-display
                    v-for="artifact in $store.getters['artifacts/allArtifacts'][position]"
                    :key="artifact.id"
                    selectable
                    :item="artifact"
                    class="artifact-item"
                    :class="{ active: selected[position] === artifact.id }"
                    @click="handleClick(position, artifact.id)"
                ></artifact-display>
            </el-tab-pane>
        </el-tabs>
    </div>
</template>

<script>
import positions from "@const/positions";
import { positionToLocale } from "@util/utils";

import ArtifactDisplay from "@c/ArtifactDisplay";

export default {
    name: "SelectArtifact",
    props: ["selected"],
    components: {
        ArtifactDisplay,
    },
    created() {
        this.positions = positions;
    },
    data() {
        return {
            activePosition: "flower",
        }
    },
    methods: {
        positionToLocale,

        handleClick(position, id) {
            if (this.selected[position] === id) {
                this.selected[position] = null;
            } else {
                this.selected[position] = id;
            }
        }
    }
}
</script>

<style lang="scss" scoped>
.artifact-panel {
    display: flex;
    flex-wrap: wrap;
    max-height: 300px;
    overflow: auto;

    .artifact-item {
        margin: 0 16px 16px 0;
        
        &.active {
            background: #3071b322;
            border-radius: 3px;
        }
    }
}


</style>