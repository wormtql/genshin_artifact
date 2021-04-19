<template>
    <div>
        <el-dialog
            title="选择buff"
            :visible.sync="selectBuffDlgVisible"
        >
            <div style="padding: 16px">
                <el-tabs>
                    <el-tab-pane
                        v-for="(genre, name) in genres"
                        :key="name"
                        :label="genre2chs[name]"
                    >
                        <div
                            v-for="buff in genre"
                            :key="buff.name"
                            class="select-buff-item flex-row"
                            @click="handleSelectBuff(buff.name)"
                        >
                            <img :src="buff.badge">
                            <span>{{ buff.chs }}</span>
                        </div>
                    </el-tab-pane>
                </el-tabs>
            </div>
        </el-dialog>

        <el-button
            type="primary"
            @click="handleAddBuff"
            size="small"
            style="margin-bottom: 16px"
        >
            <i class="el-icon-plus"></i>
            新增buff
        </el-button>

        <div
            v-for="buff in configuredBuffs"
            :key="buff.id"
            class="config-item buff-item"
            :class="{ inactive: buff.omit }"
        >
            <div class="top">
                <h3 class="config-title">{{ buff.chs }}</h3>
                <div class="buttons">
                    <el-button
                        type="text"
                        icon="el-icon-delete"
                        style="padding: 0"
                        @click="removeBuff(buff.id)"
                    ></el-button>
                    <el-button
                        type="text"
                        :icon="buff.omit ? 'el-icon-unlock' : 'el-icon-lock'"
                        style="padding: 0"
                        @click="toggleBuff(buff.id)"
                    ></el-button>
                </div>
            </div>
            
            <component
                v-if="buff.config"
                :is="buff.config"
                style="margin-top: 14px"
                ref="buffItem"
            ></component>
        </div>
    </div>
</template>

<script>
import buffs from "@asset/buff";

let genres = {};
for (let buff of Object.values(buffs)) {
    if (!genres[buff.genre]) {
        genres[buff.genre] = [];
    }

    genres[buff.genre].push(buff);
}

let genre2chs = {
    "custom": "自定义",
    "character": "角色引发的buff",
    "resonance": "元素共鸣",
    "weapon": "武器引发的buff",
};


let id = 0;

export default {
    name: "ConfigBuff",
    created() {
        this.genres = genres;
        this.genre2chs = genre2chs;
    },
    data: function () {
        return {
            configuredBuffs: [],

            selectBuffDlgVisible: false,
        }
    },
    methods: {
        toggleBuff(id) {
            for (let i = 0; i < this.configuredBuffs.length; i++) {
                let buff = this.configuredBuffs[i];

                if (buff.id === id) {
                    buff.omit = !buff.omit;
                }
            }
        },

        handleAddBuff() {
            this.selectBuffDlgVisible = true;            
        },

        handleSelectBuff(name) {
            let newBuff = {
                config: buffs[name].config,
                name,
                id: id++,
                chs: buffs[name].chs,
                omit: false,
            }

            this.configuredBuffs.push(newBuff);
            this.selectBuffDlgVisible = false;
        },

        removeBuff(id) {
            for (let i = 0; i < this.configuredBuffs.length; i++) {
                if (this.configuredBuffs[i].id === id) {
                    this.$delete(this.configuredBuffs, i);
                    break;
                }
            }
        },

        getStandardBuffs() {
            let temp = [];

            let iter = 0;
            for (let i = 0; i < this.configuredBuffs.length; i++) {
                let buff = this.configuredBuffs[i];

                if (buff.config) {
                    let comp = this.$refs.buffItem[iter++];
                    if (!buff.omit) {
                        temp.push(comp.getStandardConfig());
                    }
                } else {
                    if (!buff.omit) {
                        temp.push(buffs[buff.name].getStandardConfig());
                    }
                }
            }

            return temp;
        },

        getBuffs() {
            let temp = [];

            let iter = 0;
            for (let i = 0; i < this.configuredBuffs.length; i++) {
                let buff = this.configuredBuffs[i];

                if (buff.config) {
                    let comp = this.$refs.buffItem[iter++];

                    let buffItem = comp.getBuff();
                    buffItem.omit = buff.omit;
                    temp.push(buffItem);
                } else {
                    let buffItem = {
                        name: buff.name,
                        omit: buff.omit,
                    };
                    temp.push(buffItem);
                }
            }

            return temp;
        },

        setBuffs(arr) {
            this.configuredBuffs = [];

            let iter = 0;

            for (let buff of arr) {
                let buffDef = buffs[buff.name];

                let temp = {
                    name: buff.name,
                    id: id++,
                    chs: buffDef.chs,
                    omit: buff.omit,
                }
                if (buffDef.config) {
                    temp.config = buffDef.config;
                    this.$nextTick(() => {
                        this.$refs.buffItem[iter++].setBuff(buff.args);
                    });
                }

                this.configuredBuffs.push(temp);
            }
        }
    }
}
</script>

<style lang="scss" scoped>
.buff-item {
    margin-bottom: 16px;
    transition: 300ms;

    .top {
        display: flex;
        align-items: center;
        justify-content: space-between;

        h3 {
            margin: 0;
        }
    }

    &.inactive {
        background: #00000011;
    }
}

.select-buff-item {
    height: 48px;
    padding: 8px;
    cursor: pointer;

    &:hover {
        background: #00000011;
    }

    img {
        height: 48px;
        width: 48px;
        border-radius: 50%;
    }

    span {
        margin-left: 24px;
        font-size: 14px;
    }
}
</style>