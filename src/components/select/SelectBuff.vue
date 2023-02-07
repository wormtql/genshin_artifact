<template>
    <div>
        <el-input
            v-model="searchString"
            style="margin-bottom: 16px"
            :placeholder="t('misc.search')"
            clearable
        >
            <template slot="append">
                <i class="el-icon-search"></i>
            </template>
        </el-input>
        <el-tabs v-model="activeTab">
            <template
                v-for="genre in genres"
                :key="genre"
            >
                <el-tab-pane
                    :label="t('buffGenre', genre)"
                    :name="genre"
                    class="tab-pane mona-scroll"
                >
                    <div
                        v-for="buff in buffByGenre[genre]"
                        :key="buff.name"
                        class="buff-item"
                        @click="handleClick(buff.name)"
                    >
                        <img :src="buff.badge" class="buff-image" >
                        <div class="detail-right">
                            <p class="buff-name">{{ buff.title }}</p>
                            <p class="buff-description" v-html="buff.description"></p>
                        </div>
                        
                    </div>
                </el-tab-pane>
            </template>
            
        </el-tabs>
    </div>
</template>

<script setup lang="ts">
import { buffFlat } from "@buff"
import Fuse from "fuse.js"
import {useI18n} from "@/i18n/i18n";

const genres = [
    "Character",
    "Weapon",
    "Artifact",
    "Resonance",
    "Common",
]


// interface Props {
//     selectable: boolean,
// }
//
// const props = withDefaults(defineProps<Props>(), {
//     selectable: false
// })

interface Emits {
    (e: "select", buffName: string): void
}

const emits = defineEmits<Emits>()

const { t, ta } = useI18n()


// tab
const activeTab = ref("Character")

// buffs
interface BuffInterface {
    name: string,
    description: string,
    title: string,
    genre: string,
    badge: string,
}

const buffsFlatWithLocale = computed((): BuffInterface[] => {
    let result: BuffInterface[] = []
    for (const item of buffFlat) {
        result.push({
            name: item.name,
            title: ta(item.nameLocale),
            description: item.description ? ta(item.description) : "",
            genre: item.genre,
            badge: item.badge
        })
    }

    return result
})

const searchString = ref("")

const filteredBuffFlat = computed((): BuffInterface[] => {
    if (searchString.value === "") {
        return buffsFlatWithLocale.value
    }
    const fuse = new Fuse(buffsFlatWithLocale.value, {
        keys: ["title", "description"]
    })
    const filtered = fuse.search(searchString.value)
    return filtered.map(x => x.item) as any
})

const buffByGenre = computed(() => {
    let temp: Record<string, BuffInterface[]> = {}
    for (let item of filteredBuffFlat.value) {
        if (!Object.prototype.hasOwnProperty.call(temp, item.genre)) {
            temp[item.genre] = []
        }
        temp[item.genre].push(item)
    }
    return temp
})

function handleClick(name: string) {
    // if (!props.selectable) {
        emits("select", name)
    // }
}
</script>

<style lang="scss">
.tab-pane {
    max-height: 50vh;
    //overflow: auto;
    //
    //&::-webkit-scrollbar {
    //    width: 4px;
    //}
    //
    //&::-webkit-scrollbar-track {
    //    background: rgb(247, 247, 247);
    //    border-radius: 2px;
    //}
    //
    //&::-webkit-scrollbar-thumb {
    //    background: #d4d4d4;
    //}
}

.buff-item {
    display: flex;
    //align-items: top;
    padding: 8px;
    cursor: pointer;

    &:hover {
        background: rgb(243, 243, 243);
    }

    .buff-image {
        height: 48px;
        width: 48px;
        border-radius: 50%;
    }

    .buff-name {
        padding-left: 16px;
        margin: 0;
        font-weight: bold;
    }

    .buff-description {
        padding-left: 16px;
        margin-top: 4px;
        margin-bottom: 0;
    }
}
</style>