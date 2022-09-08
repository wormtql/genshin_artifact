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
            v-model:current-page="currentPage"
            :page-size="PAGE_SIZE"
            :total="artifactListFiltered.length"
            layout="prev, pager, next"
            :small="!deviceIsPC"
        ></el-pagination>
    </div>
</template>

<script setup lang="ts">
import { deviceIsPC } from "@util/device"

import SelectArtifactMainStat from "@c/select/SelectArtifactMainStat"
import SelectArtifactSet from "@c/select/SelectArtifactSet"
import ArtifactDisplay from "@c/display/ArtifactDisplay"
import type {ArtifactMainStatName, ArtifactPosition, ArtifactSetName, IArtifact} from "@/types/artifact"
import {useArtifactStore} from "@/store/pinia/artifact"
import {defaultArtifactSortFunction} from "@/utils/artifacts"
import {useI18n} from "@/i18n/i18n";

const PAGE_SIZE = 20

interface Props {
    position: ArtifactPosition | "any"
}

const props = withDefaults(defineProps<Props>(), {
    position: "any"
})

interface Emits {
    (e: "select", artifactId: number): void
}

const emits = defineEmits<Emits>()

const artifactStore = useArtifactStore()

// all candidate artifacts
const artifactListUnfiltered = computed((): IArtifact[] => {
    if (props.position === "any") {
        return [...artifactStore.artifacts.value.values()]
    }
    let temp = artifactStore.artifactsByPosition.value[props.position]
    // console.log(props.position)
    return temp
})


// filter
const filterMainTag = ref<ArtifactMainStatName | "any">("any")
const filterArtifactSetName = ref<ArtifactSetName | "any">("any")

const artifactListFiltered = computed((): IArtifact[] => {
    let temp: IArtifact[] = []
    for (let item of artifactListUnfiltered.value) {
        if (!(filterMainTag.value === "any" || filterMainTag.value === item.mainTag.name)) {
            continue
        }
        if (!(filterArtifactSetName.value === "any" || filterArtifactSetName.value === item.setName)) {
            continue
        }

        temp.push(item)
    }

    temp.sort(defaultArtifactSortFunction)

    return temp
})

// pager
const currentPage = ref(1)

const artifactListDisplayed = computed(() => {
    const start = (currentPage.value - 1) * PAGE_SIZE
    const end = Math.min(start + PAGE_SIZE, artifactListFiltered.value.length)

    return artifactListFiltered.value.slice(start, end)
})


// i18n
const { t } = useI18n()
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
    grid-template-columns: repeat(auto-fill, minmax(160px, 1fr));
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
        //gap: 16px;
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