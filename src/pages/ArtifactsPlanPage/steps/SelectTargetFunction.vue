<template>
    <div>
        <el-alert
            title="请注意设置目标函数参数"
            type="warning"
            :closable="false"
        ></el-alert>
        <template v-if="specificCharacterTargets">
            <h3 class="title">角色专属</h3>
            <div
                v-for="target in specificCharacterTargets"
                :key="target.name"
                class="hand item"
                :class="{ active: target.name === targetFuncName }"
                @click="handleChange(target.name)"
            >
                <div>
                    <img :src="target.badge" class="image">
                </div>
                <div class="detail">
                    <span class="target-title">{{ target.chs }}</span>
                    <span class="is-recommend" v-if="target.recommend">推荐</span>
                    <div class="description fs-12">
                        <span
                            v-for="(des, index) in target.description"
                            :key="index"
                        >
                            {{ des }}
                        </span>
                    </div>
                </div>

                <div class="tags">
                    <span
                        v-for="(tag, index) in target.tags"
                        :key="index"
                        class="tag"
                    >
                        {{ tag }}
                    </span>
                </div>
            </div>
        </template>

        <h3 class="title">通用</h3>
        <div
            v-for="target in commonTargets"
            :key="target.name"
            class="hand item"
            :class="{ active: target.name === targetFuncName }"
            @click="handleChange(target.name)"
        >
            <div>
                <img :src="target.badge" class="image">
            </div>
            <div class="detail">
                <span class="target-title">{{ target.chs }}</span>
                <span class="formula" v-if="target.formula">{{ target.formula }}</span>
                <div class="description fs-12">
                    <span
                        v-for="(des, index) in target.description"
                        :key="index"
                    >
                        {{ des }}
                    </span>
                </div>
            </div>

            <div class="tags">
                <span
                    v-for="(tag, index) in target.tags"
                    :key="index"
                    class="tag"
                >
                    {{ tag }}
                </span>
            </div>
        </div>
    </div>
</template>

<script>
import targetFunctionsData from "@asset/target_functions/data";

let targetGroup = {};
let commonTargetNames = new Set();
Object.values(targetFunctionsData).forEach(item => {
    if (!targetGroup[item["for"]]) {
        targetGroup[item["for"]] = [];
    }

    if (item["for"] === "common") {
        commonTargetNames.add(item.name);
    }
    targetGroup[item["for"]].push(item);
})

export default {
    name: "SelectTargetFunction",
    inject: ["notifyChange"],
    created: function () {
        this.commonTargets = targetGroup["common"];
    },
    data() {
        return {
            characterName: "anbo",

            targetFuncName: "single",
        }
    },
    methods: {
        getTargetFuncName() {
            return this.targetFuncName;
        },

        setTargetFuncName(characterName, targetFuncName) {
            this.characterName = characterName;
            this.targetFuncName = targetFuncName;
        },

        handleChange(name) {
            if (name !== this.targetFuncName) {
                this.targetFuncName = name;
                this.notifyChange("targetFunc", name);
            }
        },

        setCharacterName(name) {
            if (name !== this.characterName) {
                this.characterName = name;
                let newTarget = this.targetFuncName;

                if (Object.prototype.hasOwnProperty.call(targetGroup, name)) {
                    // character has own target functions

                    if (targetFunctionsData[this.targetFuncName]["for"] !== "common") {
                        let target = targetGroup[name][0];
                        newTarget = target.name;
                    }
                } else {
                    // character does not have own target functions
                    let target = targetGroup.common[0];
                    newTarget = target.name;
                }

                this.handleChange(newTarget);
            }
        }
    },
    computed: {
        specificCharacterTargets() {
            return targetGroup[this.characterName];
        }
    },
    // watch: {
    //     characterName(name) {
    //         if (commonTargetNames.has(this.value)) {
    //             return;
    //         }
    //         if (this.$parent.lock) {
    //             return;
    //         }

    //         if (Object.prototype.hasOwnProperty.call(targetGroup, name)) {
    //             let target = targetGroup[name][0];
    //             this.$emit("input", target.name);
    //         } else {
    //             // character does not have own target functions
    //             let target = targetGroup.common[0];
    //             this.$emit("input", target.name);
    //         }
    //     }
    // }
}
</script>

<style scoped lang="scss">

.title {
    /* background:rgb(74, 99, 211); */
    padding: 0px 16px;
    display: inline-block;
    /* color: white; */
    border-radius: 3px;
    font-size: 14px;
    border-bottom: 5px solid rgb(74, 99, 211);
    color: #555555;
}

.item {
    padding: 8px;
    transition: 300ms;
    border-radius: 3px;
    display: flex;
    border: 1px solid transparent;

    .image {
        width: 64px;
        display: inline-block;
        vertical-align: top;
    }

    .detail {
        display: inline-block;
        vertical-align: top;
        padding-left: 16px;
        flex: 1;

        .target-title {
            font-weight: bold;
            font-size: 14px;
            // display: block;
        }

        .is-recommend {
            font-size: 12px;
            padding: 2px 4px;
            background:rgb(255, 239, 96);
            color: rgb(0, 30, 48);
            border-radius: 3px;
            margin-left: 8px;
            display: inline-block;
        }

        .description span {
            display: block;
            color: #555555;
            padding-top: 3px;
        }

        .formula {
            display: inline-block;
            background: #d9ecff;
            font-size: 12px;
            color: #444444;
            border-radius: 3px;
            padding: 0 8px;
            /* margin-bottom: 4px; */
            margin-top: 4px;
        }
    }

    &:hover {
        background: rgba(0, 0, 0, 0.05);
    }

    .tags {
        .tag {
            font-size: 12px;
            padding: 4px 8px;
            background:cornflowerblue;
            color: white;
            border-radius: 3px;
            margin-left: 8px;
            display: inline-block;
        }
    }
}

.active {
    background: #12345611;
    border: 1px solid #12345633;
}
</style>