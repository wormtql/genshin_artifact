<template>
    <div>
        <el-breadcrumb>
            <el-breadcrumb-item>预设</el-breadcrumb-item>
        </el-breadcrumb>
        <el-divider></el-divider>

        <!-- <div class="tool-bar">
            <el-button type="primary">
                <i class="el-icon-plus"></i>
                新建
            </el-button>
        </div> -->

        <div class="body">
            <preset-item
                v-for="(item, name) in all"
                :key="name"
                :item="item"
                @delete="deletePreset(name)"
                class="item"
            ></preset-item>
        </div>
    </div>
</template>

<script>
import { mapGetters } from "vuex";

import PresetItem from "@c/display/PresetItem";

export default {
    name: "CharacterPresetsPage",
    components: {
        PresetItem,
    },
    methods: {
        deletePreset(name) {
            this.$store.commit("presets/delete", {
                name,
            });
        }
    },
    computed: {
        ...mapGetters("presets", ["all"]),
    }
}
</script>

<style scoped>
.item {
    margin: 0 16px 16px 0;
}

.body {
    display: flex;
    flex-wrap: wrap;
}
</style>