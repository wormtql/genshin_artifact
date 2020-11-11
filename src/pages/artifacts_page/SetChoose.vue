<template>
    <div class="setChoose">
        <div
            v-for="(item, index) in setName"
            :key="item"
            :class="{chooseItem: true, active: item === value}"
            @click="choose(index)"
        >
            <el-image
                style="width: 90px; height: 90px; border-radius: 5px"
                :src="publicPath + 'set/' + item + '.png'"
            >
            </el-image>
            <p>
                <!-- <i class="el-icon-check" v-if="current === item"></i> -->
                {{ toChinese[item] }}
            </p>
        </div>
        
        
    </div>
</template>

<script>
import { SET_NAME, toChinese } from "@/common/artifacts_set";

export default {
    name: "SetChoose",
    props: {
        value: {
            type: String
        }
    },
    data: function() {
        return {
            publicPath: process.env.BASE_URL,

            setName: SET_NAME,
            "toChinese": toChinese,

            // current: SET_NAME[0],
        }
    },
    methods: {
        choose: function(index) {
            this.current = SET_NAME[index];
            this.$emit("input", this.current);
        }
    }
}
</script>

<style scoped>
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

.setChoose {
    display: flex;
    flex-wrap: wrap;
    /* max-height: 300px; */
    overflow: auto;
    /* justify-content: space-between; */
}
</style>