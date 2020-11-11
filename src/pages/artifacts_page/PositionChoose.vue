<template>
    <div class="positionChoose">
        <div
            v-for="(item) in Object.keys(detailName[setName])"
            :key="item"
            :class="{chooseItem: true, active: item === value}"
            @click="choose(item)"
        >
            <el-image
                style="width: 90px; height: 90px; border-radius: 5px"
                :src="`${publicPath}detail/${setName}/${item}.png`"
            >
            </el-image>
            <p>
                <!-- <i class="el-icon-check" v-if="current === item"></i> -->
                {{ detailName[setName][item] }}
            </p>
        </div>
    </div>
</template>

<script>
import { DETAIL_NAME } from "@/common/artifacts_set";

export default {
    name: "PositionChoose",
    props: {
        setName: {
            type: String,
        },
        value: {
            type: String
        }
    },
    data: function() {
        return {
            publicPath: process.env.BASE_URL,
            detailName: DETAIL_NAME,
        }
    },
    methods: {
        choose(position) {
            this.$emit("input", position);
        }
    }
}
</script>

<style scoped>
.positionChoose {
    display: flex;
    flex-wrap: wrap;
    max-height: 300px;
    overflow: auto;
}

.chooseItem {
    text-align: center;
    font-size: 0.75rem;
    /* margin: 8px 6px; */
    padding: 6px 6px 8px 6px;
    border-radius: 5px;
    cursor: pointer;
    width: 90px;
}

.chooseItem.active {
    /* border: 1px solid #409eef; */
    background: #409eef;
    color: white;
}

.chooseItem p {
    margin: 4px 0 4px 0;
    /* padding: 0; */
}
</style>