<template>
    <div>
        <el-input
            size="medium"
            v-model="searchString"
            style="margin-bottom: 16px"
            placeholder="搜索"
            clearable
        >
            <template slot="append">
                <i class="el-icon-search"></i>
            </template>
        </el-input>
        <el-tabs v-model="activeTab">
            <template
                v-for="genre in genres"
            >
                <el-tab-pane
                    :label="genre.chs"
                    :name="genre.name"
                    :key="genre.name"
                    class="tab-pane"
                >
                    <div
                        v-for="buff in buffByGenre[genre.name]"
                        :key="buff.name"
                        class="buff-item"
                        @click="handleClick(buff.name)"
                    >
                        <img :src="buff.badge" class="buff-image" >
                        <div class="detail-right">
                            <p class="buff-name">{{ buff.chs }}</p>
                            <p class="buff-description" v-html="buff.description"></p>
                        </div>
                        
                    </div>
                </el-tab-pane>
            </template>
            
        </el-tabs>
    </div>
</template>

<script>
import { buffFlat } from "@buff"
import Fuse from "fuse.js"

const genres = [
    { name: "Character", chs: "角色引发的BUFF" },
    { name: "Weapon", chs: "武器引发的BUFF" },
    { name: "Resonance", chs: "元素共鸣" },
    { name: "Common", chs: "自定义" }
]

const fuse = new Fuse(buffFlat, {
    keys: ["chs", "description"]
})

export default {
    name: "SelectBuff",
    props: {
        selectable: {
            default: false
        }
    },
    created() {
        this.genres = genres
    },
    data() {
        return {
            activeTab: "Character",
            searchString: "",
        }
    },
    methods: {
        handleClick(name) {
            if (!this.selectable) {
                this.$emit("select", name)
            }
        }
    },
    computed: {
        filteredBuffFlat() {
            if (this.searchString === "") {
                return buffFlat
            }
            const filtered = fuse.search(this.searchString)
            return filtered.map(x => x.item)
        },

        buffByGenre() {
            let temp = {}
            for (let item of this.filteredBuffFlat) {
                if (!Object.prototype.hasOwnProperty.call(temp, item.genre)) {
                    temp[item.genre] = []
                }
                temp[item.genre].push(item)
            }
            return temp
        }
    }
}
</script>

<style lang="scss">
.tab-pane {
    max-height: 50vh;
    overflow: auto;
    
    &::-webkit-scrollbar {
        width: 4px;
    }

    &::-webkit-scrollbar-track {
        background: rgb(247, 247, 247);
        border-radius: 2px;
    }

    &::-webkit-scrollbar-thumb {
        background: #d4d4d4;
    }
}

.buff-item {
    display: flex;
    align-items: top;
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