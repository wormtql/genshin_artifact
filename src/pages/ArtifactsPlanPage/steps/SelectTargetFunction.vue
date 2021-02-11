<template>
    <div>
        <template v-if="specificCharacterTargets">
            <h3 class="title">角色专属</h3>
            <div
                v-for="target in specificCharacterTargets"
                :key="target.name"
                class="hand item"
                @click="handleClick(target.name)"
            >
                <img :src="target.badge" class="image">
                <div class="detail">
                    <span class="target-title">{{ target.chs }}</span>
                    <div class="description">
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
            @click="handleClick(target.name)"
        >
            <img :src="target.badge" class="image">
            <div class="detail">
                <span class="target-title">{{ target.chs }}</span>
                <div class="description">
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
import { targetFunctionsData } from "../../../assets/target_functions";

let targetGroup = {};
Object.values(targetFunctionsData).forEach(item => {
    if (!targetGroup[item["for"]]) {
        targetGroup[item["for"]] = [];   
    }

    targetGroup[item["for"]].push(item);
})

export default {
    name: "SelectTargetFunction",
    created: function () {
        this.commonTargets = targetGroup["common"];
    },
    props: {
        characterName: {
            type: String,
            required: true,
        }
    },
    methods: {
        handleClick(targetFunctionName) {
            this.$emit("select", targetFunctionName);
        }
    },
    computed: {
        specificCharacterTargets() {
            return targetGroup[this.characterName];
        }
    }
}
</script>

<style scoped>
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

.image {
    width: 100px;
    display: inline-block;
    vertical-align: top;
}

.detail {
    display: inline-block;
    vertical-align: top;
    padding-left: 16px;
    flex: 1;
}

.target-title {
    font-weight: bold;
    font-size: 14px;
}

.description span {
    display: block;
    font-size: 12px;
    color: #555555;
    padding-top: 3px;
}

.item {
    padding: 8px;
    transition: 300ms;
    border-radius: 3px;
    display: flex;
}

.item:hover {
    background: rgba(0, 0, 0, 0.05);
}

.tag {
    font-size: 12px;
    padding: 4px 8px;
    background:cornflowerblue;
    color: white;
    border-radius: 3px;
    margin-left: 8px;
    display: inline-block;
}
</style>