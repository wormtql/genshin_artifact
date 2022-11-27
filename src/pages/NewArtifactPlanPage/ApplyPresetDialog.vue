<template>
    <el-dialog
        title="导入预设"
        v-model="visible"
        width="80%"
    >
        <div class="body">
            <preset-item
                v-for="entry in presetsAllFlat"
                :key="entry.name"
                :toolbar="false"
                :item="entry.item"
                :name="entry.name"
                class="hand item"
                @click="handleClick(entry.name)"
            ></preset-item>
        </div>
    </el-dialog>
</template>

<!-- <script>
import PresetItem from "@c/display/PresetItem";

import { mapGetters } from "vuex";

export default {
    name: "ApplyPresetDialog",
    components: {
        PresetItem,
    },
    data() {
        return {
            visible: false,
        }
    },
    methods: {
        open() {
            this.visible = true;
        },

        handleClick(name) {
            this.visible = false;
            this.$emit("selected", name);
        }
    },
    computed: {
        ...mapGetters("presets", {
            presetsAllFlat: "allFlat"
        }),
    }
}
</script> -->

<script setup lang="ts">
import PresetItem from "@/components/display/PresetItem.vue"
import {usePresetStore} from "@/store/pinia/preset"

const presetStore = usePresetStore()
const presetsAllFlat = presetStore.allFlat

const emit = defineEmits<{
    (e: 'selected', name: string): void
}>()

const visible = ref(false)

function open() {
    visible.value = true
}

function handleClick(name: string) {
    visible.value = false
    emit('selected', name)
}

defineExpose({
    open,
})
</script>

<style scoped lang="scss">
.active {
    background: #12345611;
}

.footer {
    display: flex;
    flex-direction: row-reverse;
}

.body {
    display: grid;
    gap: 4px;
    grid-template-columns: repeat(auto-fill, minmax(230px, 1fr));
    grid-auto-rows: min-content;

    .item {
        width: 100%;
    }
}
</style>
