<template>
    <div>
        <el-breadcrumb>
            <el-breadcrumb-item>全队配装</el-breadcrumb-item>
        </el-breadcrumb>
        <el-divider></el-divider>

        <el-row :gutter="16">
            <el-col
                :span="6"
                ref="content"
                :style="{ height: contentHeight }"
                class="mona-scroll"
            >
                <div
                    style="margin-bottom: 12px"
                >
                    <my-button1
                        icon="el-icon-caret-right"
                        title="开始计算"
                        @click="handleClickStart"
                    ></my-button1>
                </div>

                <div
                    v-for="(characterName, index) in characterNames"
                    :key="index"
                    class="member-item"
                >
                    <h3 class="title">成员{{ index + 1 }}</h3>
<!--                    <el-row :gutter="16">-->
<!--                        <el-col-->
<!--                            :span="12"-->
<!--                        >-->
                    <select-character
                        :value="characterNames[index]"
                        @input="handleChangeCharacterName(index, $event)"
                    ></select-character>
                    <item-config
                        v-if="characterConfigs[index] !== 'NoConfig'"
                        v-model="characterConfigs[index]"
                        :item-name="characterNames[index]"
                        :configs="characterConfigConfigs[index]"
                        style="margin-top: 12px"
                    ></item-config>
                    <div>
                        <img class="image" :src="characterAvatars[index]" />
                    </div>

                    <div>
                        <h3 class="common-title2">命之座</h3>
                        <el-input-number
                            size="small"
                            v-model="characterConstellations[index]"
                            :min="0"
                            :max="6"
                        ></el-input-number>
                    </div>

                    <div>
                        <h3 class="common-title2">武器</h3>
                        <select-weapon
                            :value="weaponNames[index]"
                            @input="handleChangeWeaponName(index, $event)"
                            :type="characterWeaponTypes[index]"
                            style="margin-bottom: 12px;"
                        ></select-weapon>
                        <item-config
                            v-if="weaponConfigs[index] !== 'NoConfig'"
                            v-model="weaponConfigs[index]"
                            :item-name="weaponNames[index]"
                            :configs="weaponConfigConfigs[index]"
                        ></item-config>
                        <div>
                            <img class="image" :src="weaponImages[index]" />
                        </div>
                    </div>

                    <h3 class="common-title2">精炼</h3>
                    <el-input-number
                        size="small"
                        v-model="weaponRefines[index]"
                        :min="1"
                        :max="5"
                    ></el-input-number>

                    <el-divider v-if="index !== characterNames.length - 1"></el-divider>
<!--                        </el-col>-->
<!--                    </el-row>-->
                </div>
            </el-col>

            <el-col
                :span="18"
            >
                <el-alert
                    v-if="maybeTeamChs"
                    :closable="false"
                    :title="`检测到队伍名「${maybeTeamChs}」`"
                ></el-alert>
                <template v-if="currentResultEntry">
                    <el-input-number
                        :value="resultIndex + 1"
                        @input="handleChangeResultIndex"
                        :min="1"
                        :max="results.length"
                        size="small"
                        style="margin-bottom: 12px"
                    ></el-input-number>
                    <div
                        v-for="(characterName, index) in characterNames"
                        :key="index"
                        class="result-item"
                    >
                        <div class="result-item-top">
                            <div>
                                <span class="result-item-title">{{ characterChs[index] }}</span>
                            </div>

                            <div class="result-item-buttons">
                                <el-button
                                    icon="el-icon-plus"
                                    circle
                                    size="mini"
                                    type="text"
                                    title="存为套装"
                                ></el-button>
                            </div>
                        </div>
                        <div class="result-item-content">
                            <artifact-display
                                v-for="artifactId in currentResultEntry[index]"
                                :key="artifactId"
                                :item="artifactsById[artifactId]"
                            ></artifact-display>
                        </div>
                    </div>
                </template>
                <template v-else>
                    <el-empty></el-empty>
                </template>
            </el-col>
        </el-row>
    </div>
</template>

<script>
import { mapGetters } from "vuex"

import { getWeaponDefaultConfigWasmInterface } from "@util/weapon"
import { getCharacterDefaultConfigWasmInterface } from "@util/character"
import { characterData } from "@asset/character"
import { weaponData } from "@asset/weapon"
import { convertArtifact } from "@util/converter"
import { team_optimize } from "@/wasm"

import SelectCharacter from "@c/select/SelectCharacter"
import SelectWeapon from "@c/select/SelectWeapon"
import ItemConfig from "@c/config/ItemConfig"
import ArtifactDisplay from "@c/display/ArtifactDisplay"
import MyButton1 from "@c/button/MyButton1"

export default {
    name: "TeamOptimizationPage",
    components: {
        SelectCharacter,
        SelectWeapon,
        ItemConfig,
        ArtifactDisplay,
        MyButton1,
    },
    data() {
        return {
            characterNames: [ "RaidenShogun", "KujouSara", "KaedeharaKazuha", "Bennett"],
            characterConfigs: ["NoConfig", "NoConfig", "NoConfig", "NoConfig"],
            characterConstellations: [2, 6, 0, 1],

            weaponNames: ["EngulfingLightning", "FavoniusWarbow", "FreedomSworn", "FesteringDesire"],
            weaponConfigs: ["NoConfig", "NoConfig", "NoConfig", "NoConfig"],
            weaponRefines: [1, 1, 1, 5],

            results: [],    // a 3d array
            resultIndex: 0,

            contentHeight: "",
        }
    },
    created() {
        for (let i = 0; i < this.weaponNames.length; i++) {
            const weaponName = this.weaponNames[i]
            const defaultConfig = getWeaponDefaultConfigWasmInterface(weaponName)
            this.$set(this.weaponConfigs, i, defaultConfig)
        }

        for (let i = 0; i < this.characterNames.length; i++) {
            const characterName = this.characterNames[i]
            const defaultConfig = getCharacterDefaultConfigWasmInterface(characterName)
            this.$set(this.characterConfigs, i, defaultConfig)
        }
        // console.log(this.weaponConfigs)
        // console.log(this.characterConfigs)
    },
    mounted() {
        const component = this.$refs["content"]

        this.$nextTick(() => {
            const rect = component.$el.getBoundingClientRect()
            console.log(rect)
            this.contentHeight = `calc(100vh - ${rect.top}px)`
        })
    },
    methods: {
        handleClickStart() {
            const interfaceWasm = this.optimizeTeamWasmInterface

            const loading = this.$loading({
                lock: true,
                text: "莫娜占卜中（可能需要数分钟）"
            })

            const closeLoading = () => {
                loading.close()
            }

            team_optimize(interfaceWasm).then(result => {
                console.log(result)
                this.results = result.artifacts
                this.resultIndex = 0
            }).catch(e => {
                console.log(e)
            }).finally(() => {
                closeLoading()
            })
            // console.log(interfaceWasm)
            // let result = this.$mona.TeamOptimizationWasm.optimize_team(interfaceWasm)
            // console.log(result)
            // this.results = result.artifacts
            // this.resultIndex = 0
        },

        handleChangeWeaponName(index, name) {
            if (name === this.weaponNames[index]) {
                return
            }

            const defaultConfig = getWeaponDefaultConfigWasmInterface(name)
            this.$set(this.weaponConfigs, index, defaultConfig)
            this.$set(this.weaponNames, index, name)
        },

        handleChangeCharacterName(index, name) {
            if (name === this.characterNames[index]) {
                return
            }

            const defaultConfig = getCharacterDefaultConfigWasmInterface(name)
            this.$set(this.characterConfigs, index, defaultConfig)
            this.$set(this.characterNames, index, name)
        },

        handleChangeResultIndex(index) {
            this.resultIndex = index - 1
        }
    },
    computed: {
        ...mapGetters("artifacts", {
            artifactsFlat: "allFlat",
            artifactsById: "artifactsById",
        }),

        maybeTeamChs() {
            const maybeName = this.$mona.TeamOptimizationWasm.match_name(this.optimizeTeamWasmInterface)
            // console.log(maybeName)
            return maybeName
        },

        currentResultEntry() {
            if (this.results.length === 0) {
                return null
            } else {
                return this.results[this.resultIndex]
            }
        },

        filteredArtifacts() {
            let results = []
            for (let artifact of this.artifactsFlat) {
                if (artifact.level >= 16) {
                    results.push(artifact)
                }
            }
            return results
        },

        filteredArtifactsWasm() {
            return this.filteredArtifacts.map(convertArtifact)
        },

        characterConfigConfigs() {
            let results = []
            for (let i = 0; i < this.characterNames.length; i++) {
                const name = this.characterNames[i]
                const data = characterData[name]
                results.push(data.config ?? [])
            }
            return results
        },

        characterWeaponTypes() {
            let results = []
            for (let i = 0; i < this.characterNames.length; i++) {
                const name = this.characterNames[i]
                const data = characterData[name]
                results.push(data.weapon)
            }
            return results
        },

        characterAvatars() {
            let results = []
            for (let i = 0; i < this.characterNames.length; i++) {
                const name = this.characterNames[i]
                const data = characterData[name]
                results.push(data.avatar)
            }
            return results
        },

        characterChs() {
            let results = []
            for (let i = 0; i < this.characterNames.length; i++) {
                const name = this.characterNames[i]
                const data = characterData[name]
                results.push(data.chs)
            }
            return results
        },

        characterInterfaces() {
            let results = []
            for (let i = 0; i < this.characterNames.length; i++) {
                results.push({
                    name: this.characterNames[i],
                    level: 90,
                    ascend: false,
                    constellation: 0,
                    skill1: 8,
                    skill2: 8,
                    skill3: 8,
                    params: this.characterConfigs[i]
                })
            }
            return results
        },

        weaponConfigConfigs() {
            let results = []
            for (let i = 0; i < this.weaponNames.length; i++) {
                const name = this.weaponNames[i]
                const data = weaponData[name]
                results.push(data.configs ?? [])
            }
            return results
        },

        weaponImages() {
            let results = []
            for (let i = 0; i < this.weaponNames.length; i++) {
                const name = this.weaponNames[i]
                const data = weaponData[name]
                results.push(data.url)
            }
            return results
        },

        weaponInterfaces() {
            let results = []
            for (let i = 0; i < this.weaponNames.length; i++) {
                results.push({
                    name: this.weaponNames[i],
                    level: 90,
                    ascend: false,
                    refine: 0,
                    params: this.weaponConfigs[i]
                })
            }
            return results
        },

        teamInterface() {
            return {
                characters: this.characterInterfaces,
                weapons: this.weaponInterfaces
            }
        },

        optimizeTeamHyperParamInterface() {
            // todo
            return {
                mva_step: 5,
                work_space: 1000,
                max_re_optimize: 5,
                max_search: 2000000,
                count: 1000,
            }
        },

        optimizeTeamWasmInterface() {
            return {
                team: this.teamInterface,
                team_target_function_config: null,      // todo
                override_target_functions: null,        // todo
                artifacts: this.filteredArtifactsWasm,
                hyper_param: this.optimizeTeamHyperParamInterface
            }
        }
    },
    watch: {
        // characterNames: {
        //     handler: function (newNames, oldNames) {
        //         const oldLen = oldNames.length
        //         const newLen = newNames.length
        //
        //         if (oldLen !== newLen) {
        //             return
        //         }
        //
        //         for (let i = 0; i < oldLen; i++) {
        //             const oldName = oldNames[i]
        //             const newName = newNames[i]
        //             if (oldName !== newName) {
        //                 const defaultConfig = getCharacterDefaultConfigWasmInterface(newName)
        //                 this.$set(this.characterConfigs, i, defaultConfig)
        //             }
        //         }
        //     },
        //     deep: true,
        // },
        //
        // weaponNames: {
        //     handler: function (newNames, oldNames) {
        //         const oldLen = oldNames.length
        //         const newLen = newNames.length
        //
        //         if (oldLen !== newLen) {
        //             return
        //         }
        //         console.log(oldNames, newNames)
        //
        //         for (let i = 0; i < oldLen; i++) {
        //             const oldName = oldNames[i]
        //             const newName = newNames[i]
        //             console.log(oldName, newName)
        //             if (oldName !== newName) {
        //                 const defaultConfig = getWeaponDefaultConfigWasmInterface(newName)
        //
        //                 this.$set(this.weaponConfigs, i, defaultConfig)
        //             }
        //         }
        //
        //         console.log(this.weaponConfigs)
        //     },
        //     deep: true,
        // }
    }
}
</script>

<style scoped lang="scss">
.member-item {
    margin-bottom: 16px;
    //box-shadow: 0 0 10px 1px #00000011;
    //padding: 12px;

    &:last-of-type {
        margin-bottom: 64px;
    }

    .title {
        font-size: 14px;
        margin: 0;
        margin-bottom: 12px;
    }

    .image {
        width: 64px;
        height: 64px;
        display: inline-block;
        border-radius: 50%;
        margin-top: 12px;
    }
}

.common-title2 {
    font-size: 12px;
    color: #666666;
}

.result-item {
    margin-bottom: 12px;

    &:last-of-type {
        margin-bottom: 0;
    }

    .result-item-top {
        display: flex;
        align-items: center;
        justify-content: space-between;
        height: 36px;
        border-bottom: 1px solid #00000011;

        .result-item-title {
            font-size: 12px;
        }

        .result-item-buttons {
            display: flex;
            align-items: center;
        }
    }

    .result-item-content {
        padding-top: 12px;
        display: flex;
        flex-wrap: wrap;
        gap: 12px;
    }
}
</style>