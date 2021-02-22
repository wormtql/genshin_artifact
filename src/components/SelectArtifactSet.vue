<template>
    <el-select
        @input="$emit('input', $event)"
        :value="value"
        :disabled="disabled"
    >
        <el-option
            v-for="item in allArtifactsName"
            :key="item.name"
            :label="item.chs"
            :value="item.name"
        >
            <div class="item">
                <img :src="item.url" class="tn">
                <span>{{ item.chs }}</span>
            </div>
        </el-option>
    </el-select>
</template>

<script>
import { artifactsData } from "@asset/artifacts";
import { getArtifactThumbnailURL } from "@util/utils";

let allArtifacts = Object.values(artifactsData);
allArtifacts.sort((a, b) => {
    return b.maxStar - a.maxStar;
});
let allArtifactsName = allArtifacts.map(item => {
    return {
        name: item.eng,
        chs: item.chs,
        url: getArtifactThumbnailURL(item.eng),
    };
});


export default {
    name: "SelectArtifactSet",
    created: function () {
        this.allArtifactsName = allArtifactsName;
    },
    props: {
        value: {
            type: String,
            required: true,
        },
        disabled: {
            type: Boolean,
            default: false,
        }
    }
}
</script>

<style scoped>
.tn {
    height: 80%;
    border-radius: 50%;
    display: inline-block;
    margin-right: 8px;
}

.item {
    display: flex;
    align-items: center;
    height: 100%;
}
</style>