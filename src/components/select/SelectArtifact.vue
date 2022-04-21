<template>
    <div>
        <div class="filters">
<!--            <span>主词条：</span>-->
            <select-artifact-main-stat
                v-model="filterMainTag"
                placeholder="主词条"
            ></select-artifact-main-stat>
<!--            <span>套装：</span>-->
            <select-artifact-set
                v-model="filterArtifactSetName"
                any-option
                placeholder="套装"
            ></select-artifact-set>
        </div>

        <div class="artifacts" v-show="artifactListDisplayed.length > 0">
            <artifact-display
                v-for="artifact in artifactListDisplayed"
                :key="artifact.id"
                :item="artifact"
                selectable
                @click="$emit('select', artifact.id)"
            ></artifact-display>
        </div>
        <div v-show="artifactListDisplayed.length === 0" class="no-artifacts">
            <el-empty></el-empty>
        </div>

        <el-pagination
            :current-page.sync="currentPage"
            :page-size="pageSize"
            :total="artifactList.length"
            layout="prev, pager, next"
            :small="!deviceIsPC"
        ></el-pagination>
    </div>
</template>

<script>
import { deviceIsPC } from "@util/device"

import SelectArtifactMainStat from "@c/select/SelectArtifactMainStat"
import SelectArtifactSet from "@c/select/SelectArtifactSet"
import ArtifactDisplay from "@c/display/ArtifactDisplay"

const pageSize = 20

export default {
    name: "SelectArtifact",
    components: {
        SelectArtifactMainStat,
        SelectArtifactSet,
        ArtifactDisplay
    },
    props: {
        position: {}
    },
    created() {
        this.pageSize = pageSize
    },
    data() {
        return {
            filterMainTag: "any",
            filterArtifactSetName: "any",

            currentPage: 1,

            deviceIsPC
        }
    },
    computed: {
        artifactListUnfiltered() {
            if (this.position === "any") {
                return this.$store.getters["artifacts/allFlat"]
            }
            const ret = this.$store.state.artifacts[this.position]
            return ret
        },

        artifactList() {
            let temp = []
            for (let item of this.artifactListUnfiltered) {
                if (!(this.filterMainTag === "any" || this.filterMainTag === item.mainTag.name)) {
                    continue
                }
                if (!(this.filterArtifactSetName === "any" || this.filterArtifactSetName === item.setName)) {
                    continue
                }

                temp.push(item)
            }

            return temp
        },

        artifactListDisplayed() {
            const start = (this.currentPage - 1) * pageSize
            const end = Math.min(start + pageSize, this.artifactList.length)

            return this.artifactList.slice(start, end)
        }
    }
}
</script>

<style lang="scss" scoped>
.artifacts {
    //display: flex;
    //gap: 16px;
    //flex-wrap: wrap;
    //margin: 16px 0;
    //

    margin: 16px 0;
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(160px, 1fr));
    grid-auto-rows: max-content;
    gap: 4px;
}

.no-artifacts {
    min-height: 200px;
    display: flex;
    align-items: center;
    justify-content: center;
}

@media only screen and (min-width: 992px) {
    .filters {
        display: flex;
        align-items: center;
        gap: 16px;
    }
}

@media only screen and (max-width: 992px) {
    .filters {
        .el-select {
            width: 100%;
        }
    }
}

</style>