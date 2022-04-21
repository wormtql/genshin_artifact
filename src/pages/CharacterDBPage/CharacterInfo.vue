<template>
    <div>
        <el-empty v-if="!loaded"></el-empty>
        <div v-else class="tab-full-height mona-scroll-hidden content-div">
            <p class="analysis-item-title">武器使用率</p>
            <w-c-bar
                v-for="item in characterResult[this.characterName].weapon_usage"
                :key="item[0]"
                :item="item"
                type="weapon"
            ></w-c-bar>

            <p class="analysis-item-title">最佳圣遗物</p>
            <div
                v-for="(item, index) in characterResult[this.characterName].artifact_set_usage"
                :key="index"
                class="bar-item-artifact"
            >
                <artifact-bar :item="item"></artifact-bar>
            </div>

            {{ characterResult[this.characterName] }}

            <p class="analysis-item-title">最佳主词条</p>
            <div>

            </div>
        </div>
    </div>
</template>

<script>
import {characterByElement} from "@character"
import {getComputeResultAnalysis} from "@/api/misc"

import ArtifactBar from "./ArtifactBar"
import WCBar from "./WCBar"

export default {
    name: "MonaDBCharacter",
    components: {
        ArtifactBar,
        WCBar
    },
    data() {
        return {
            analysisResult: null,
            characterName: "Amber",

            loaded: false,
            error: false,

            characterByElement
        }
    },
    mounted() {
        this.characterName = this.$route.params.name ?? "Amber"
        this.refresh(this.characterName)
    },
    methods: {
        refresh(name) {
            this.loaded = false

            getComputeResultAnalysis().then(result => {
                this.analysisResult = result
                this.error = false
            }).catch(e => {
                console.log(e)
                this.error = true
            }).finally(() => {
                this.loaded = true
            })
        },
    },
    computed: {
        characterResult() {
            if (this.analysisResult) {
                return this.analysisResult.character_result
            }

            return null
        }
    }
}
</script>

<style scoped>

</style>