<template>
    <div>
        <el-tabs v-model="activeName" type="card" :stretch="true">
            <el-tab-pane
                v-for="artType in artifactsType"
                :key="artType.name"
                class="panel"
                :name="artType.name"
            >
                <div slot="label" class="flex-row">
                    <img :src="artType.iconURL">
                    <span>{{ artType.chs }}</span>
                </div>
                <artifact-display
                    class="artifact-panel"
                    v-for="(item, index) in filteredArtifacts[artType.name]"
                    :key="artType.name + item.id"
                    :item="item"
                    :selectable="true"
                    @click="handleClick(artType.name, index)"
                ></artifact-display>
            </el-tab-pane>
        </el-tabs>
    </div>
</template>

<script>
import ArtifactDisplay from "@c/ArtifactDisplay";

import { artifactsIcon } from "@asset/artifacts";

export default {
    name: "SelectArtifact",
    components: {
        ArtifactDisplay,
    },
    data: function () {
        return {
            activeName: "flower",
        }
    },
    created: function () {
        this.artifactsType = [
            { name: "flower", chs: "生之花", iconURL: artifactsIcon["flower"] },
            { name: "feather", chs: "死之羽", iconURL: artifactsIcon["feather"] },
            { name: "sand", chs: "时之沙", iconURL: artifactsIcon["sand"] },
            { name: "cup", chs: "空之杯", iconURL: artifactsIcon["cup"] },
            { name: "head", chs: "理之冠", iconURL: artifactsIcon["head"] }
        ];
    },
    methods: {
        handleClick(position, index) {
            let art = this.filteredArtifacts[position][index];
            this.$emit("select", art);
        }
    },
    computed: {
        filteredArtifacts() {
            let allArtifacts = this.$store.getters["artifacts/allArtifacts"];
            let fil = art => (art.star || 5) === 5;
            let temp = {};
            for (let i in allArtifacts) {
                temp[i] = allArtifacts[i].filter(fil);
            }

            return temp;
        }
    }
}
</script>

<style scoped>
.panel {
    display: flex;
    flex-wrap: wrap;

}

.artifact-panel {
    margin: 8px;
}
</style>