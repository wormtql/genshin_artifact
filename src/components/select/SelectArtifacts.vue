<template>
    <div>
        <artifacts-filter
            :filter.sync="artifactsFilter"
            style="margin-bottom: 16px"
        ></artifacts-filter>

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
                    v-for="artifact in filteredArtifacts[position]"
                    :key="artifact.id"
                    selectable
                    :item="artifact"
                    class="artifact-item"
                    :class="{ active: selected[position] === artifact.id }"
                    @click="handleClick(position, artifact.id)"
                    width="200px"
                ></artifact-display>
                <el-alert
                    title="没有符合条件的圣遗物"
                    v-if="filteredArtifacts[position].length === 0"
                    :closable="false"
                ></el-alert>
            </el-tab-pane>
        </el-tabs>
    </div>
</template>

<script>
import positions from "@const/positions";
import { positionToLocale } from "@util/utils";

import ArtifactDisplay from "@c/ArtifactDisplay";
import ArtifactsFilter from "@c/filter/ArtifactsFilter";

export default {
    name: "SelectArtifact",
    props: ["selected"],
    components: {
        ArtifactDisplay,
        ArtifactsFilter,
    },
    created() {
        this.positions = positions;
    },
    data() {
        return {
            activePosition: "flower",
            artifactsFilter: () => true,
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
    },
    computed: {
        filteredArtifacts() {
            let temp = {};
            for (let pos of positions) {
                temp[pos] = this.$store.getters['artifacts/allArtifacts'][pos].filter(this.artifactsFilter);
            }
            return temp;
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