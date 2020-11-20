<template>
    <el-card :style="cardStyle" shadow="never">
        <div slot="header" class="header">
            <!-- <span>{{ item.setName }}</span> -->
            <span>{{ item.detailName }}</span>
            <!-- <span>{{ item.position }}</span> -->

            <div v-if="canDelete">
                <el-button
                    class="menuButton"
                    circle type="text" icon="el-icon-delete"
                    @click="$emit('delete', item)"
                ></el-button>
                <el-button
                    class="menuButton"
                    circle
                    type="text"
                    icon="el-icon-crop"
                    @click="$emit('toggle', item)"
                >
                </el-button>
                <!-- <el-button
                    class="menuButton"
                    circle type="text" icon="el-icon-edit"
                ></el-button> -->
            </div>
        </div>
        
        <el-row>
            <el-col :span="10">
                <el-image
                    :src="`${publicPath}detail/${item.setName}/${item.position}.png`"
                    style="width: 70px; height: 70px"
                ></el-image>
            </el-col>
            <el-col :span="14">
                <h3 class="primaryTag">{{ displayTag(item.primary.tag, item.primary.value) }}</h3>

                <el-tag
                    size="mini"
                    class="secondaryTag"
                    v-for="(tag, index) in item.secondary"
                    :key="index"
                >
                    {{ displayTag(tag.tag, tag.value) }}
                </el-tag>
            </el-col>
        </el-row>
        
    </el-card>
</template>

<script>
import { displayTag } from "@/utils";

export default {
    name: "Artifact",
    props: {
        disabled: {
            type: Boolean,
            default: false,
        },
        item: {
            type: Object,
            default: () => ({
                setName: "luckyDog",
                position: "cup",
                detailName: "冒险家的金杯",
                primary: {
                    tag: "attack1",
                    value: 100,
                },
                secondary: [
                    { tag: "defend1", value: 20, },
                    { tag: "attack2", value: 30 },
                ],
                omit: false,
            })
        },
        canDelete: {
            type: Boolean,
            default: true,
        },
        omit: {
            type: Boolean,
            default: false,
        }
    },
    data: function() {
        return {
            publicPath: process.env.BASE_URL,

            // omitState: false,
        }
    },
    methods: {
        displayTag,
    },
    computed: {
        cardStyle: function() {
            // window.console.log("123");
            return {
                width: "250px",
                // height: "200px",
                background: this.omit ? "#00000011" : "unset",
            };
        }
    }
}
</script>

<style scoped>
.header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    /* background: red; */
}

.menuButton {
    padding: 0 0 0 8px;
    margin: 0;
}

.primaryTag {
    padding: 0;
    margin: 0;
    font-size: 0.9rem;
}

.secondaryTag {
    margin: 0 4px 2px 0;
    /* padding: 0;
    margin: 0;
    font-size: 0.7rem; */
}
</style>