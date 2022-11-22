<template>
    <div class="content-div">
        <el-tabs v-model="activeTab">
            <el-tab-pane
                v-for="(characters, elementName) in characterByElement"
                :key="elementName"
                :label="t('ele', elementName)"
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
import {useI18n} from "../../i18n/i18n";

export default {
    name: "MonaDBPage",
    data() {
        return {
            activeTab: "Pyro",

            characterByElement
        }
    },
    methods: {
        handleClickCharacter(e) {
            const element = e.target
            if (element.hasAttribute("x-character-name")) {
                const characterName = element.getAttribute("x-character-name")
                this.$router.push(`/character/${characterName}`)
            }
        }
    },
    setup() {
        const { t } = useI18n()

        return {
            t
        }
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
    grid-auto-rows: min-content;

    img {
        width: 100%;
    }
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


</style>