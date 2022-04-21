<template>
    <div class="content-div">
        <el-tabs v-model="activeTab">
            <el-tab-pane
                v-for="(characters, elementName) in characterByElement"
                :key="elementName"
                :label="element2Chs(elementName)"
                :name="elementName"
                class="character-tab-pane"
                @click.native="handleClickCharacter"
            >
                <img
                    v-for="character in characters"
                    :key="character.name"
                    class="character-image"
                    :src="character.avatar"
                    :x-character-name="character.name"
                >
            </el-tab-pane>
        </el-tabs>

        <router-view class="content-view"></router-view>
    </div>
</template>

<script>
import { characterByElement } from "@character"
import { element2Chs } from "@util/common"

export default {
    name: "MonaDBPage",
    data() {
        return {
            activeTab: "Pyro",

            characterByElement
        }
    },
    methods: {
        element2Chs,

        handleClickCharacter(e) {
            const element = e.target
            if (element.hasAttribute("x-character-name")) {
                const characterName = element.getAttribute("x-character-name")
                this.$router.push(`/character/${characterName}`)
            }
        }
    },
    computed: {
    }
}
</script>

<style lang="scss" scoped>
.character-tab-pane {
    display: grid;
    //gap: 8px;
    //grid-template-columns: repeat(auto-fill, 80px);
    //grid-auto-columns: minmax(64px, 100px);

    gap: 4px;
    grid-template-columns: repeat(auto-fill, minmax(64px, 1fr));
}

//@media only screen and (max-width: 1200px) {
//    .character-tab-pane {
//        grid-template-columns: repeat(8, 1fr);
//    }
//}
//
//@media only screen and (max-width: 992px) {
//    .character-tab-pane {
//        grid-template-columns: repeat(6, 1fr);
//    }
//}
//
//@media only screen and (max-width: 768px) {
//    .character-tab-pane {
//        grid-template-columns: 1fr 1fr 1fr 1fr;
//        grid-auto-columns: 1fr;
//    }
//}

.character-image {
    //width: 100%;
    width: 100%;
    //max-width: 100%;
    //height: auto;
    //height: 64px;
    border-radius: 5px;
    //border: 1px solid #79bbff;
}

.list-group-title {
    color: #909399;
    font-size: 12px;
}

@media only screen and (min-width: 992px) {
    .content-div {
        margin: 0 150px;
        min-width: 60%;
    }
}


.list-item {
    height: 48px;
    padding: 6px;
    display: flex;
    align-items: center;
    transition: 300ms;
    border-left: 3px solid transparent;

    &:hover {
        background-color: #f5f7fa;
        cursor: pointer;
    }

    &.active {
        border-left: 3px solid #79bbff;
        background-color: #ecf5ff;
    }

    .list-item-image {
        height: 48px;
        width: 48px;
        border-radius: 50%;
        pointer-events: none;
    }

    .list-item-title {
        font-size: 14px;
        color: #606266;
        margin-left: 24px;
        pointer-events: none;
    }
}

.bar-item {
    display: flex;
    align-items: center;

    .graph-label-image {
        width: 48px;
        height: 48px;
        border-radius: 50%;
        margin-right: 12px;
    }
}

.list-item-bar-h {
    display: flex;
    align-items: center;

    .list-item-group-h {
        display: flex;
        align-items: center;

        .list-group-title {
            margin-right: 12px;
        }

        .list-item-h {
            padding: 4px;
            border-bottom: 3px solid transparent;
            &.active {
                background-color: #ecf5ff;
                border-bottom: 3px solid #79bbff;
            }
        }

        .list-item-image {
            width: 48px;
            height: 48px;
            border-radius: 50%;
            pointer-events: none;
        }
    }
}


</style>