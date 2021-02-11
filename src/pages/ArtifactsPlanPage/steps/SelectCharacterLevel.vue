<template>
    <div>
        <div class="panel">
            <span
                v-for="i in 19"
                :key="i"
                class="item"
                @click="handleClickLevel(i)"
            >{{ i }}</span>
            <span class="item special" @click="handleClickLevel('20-')">20-</span>
        </div>
        
        <div class="panel">
            <span class="item special" @click="handleClickLevel('20+')">20+</span>
            <span
                v-for="i in 19"
                :key="i + 20"
                class="item"
                @click="handleClickLevel(i + 20)"
            >{{ i + 20 }}</span>
            <span class="item special" @click="handleClickLevel('40-')">40-</span>
        </div>

        <div class="panel">
            <span class="item special" @click="handleClickLevel('40+')">40+</span>
            <span
                v-for="i in 9"
                :key="i + 40"
                class="item"
                @click="handleClickLevel(i + 40)"
            >{{ i + 40 }}</span>
            <span class="item special" @click="handleClickLevel('50-')">50-</span>
        </div>

        <div class="panel">
            <span class="item special" @click="handleClickLevel('50+')">50+</span>
            <span
                v-for="i in 9"
                :key="i + 50"
                class="item"
                @click="handleClickLevel(i + 50)"
            >{{ i + 50 }}</span>
            <span class="item special" @click="handleClickLevel('60-')">60-</span>
        </div>

        <div class="panel">
            <span class="item special" @click="handleClickLevel('60+')">60+</span>
            <span
                v-for="i in 9"
                :key="i + 60"
                class="item"
                @click="handleClickLevel(i + 60)"
            >{{ i + 60 }}</span>
            <span class="item special" @click="handleClickLevel('70-')">70-</span>
        </div>

        <div class="panel">
            <span class="item special" @click="handleClickLevel('70+')">70+</span>
            <span
                v-for="i in 9"
                :key="i + 70"
                class="item"
                @click="handleClickLevel(i + 70)"
            >{{ i + 70 }}</span>
            <span class="item special" @click="handleClickLevel('80-')">80-</span>
        </div>

        <div class="panel">
            <span class="item special" @click="handleClickLevel('80+')">80+</span>
            <span
                v-for="i in 10"
                :key="i + 80"
                class="item"
                @click="handleClickLevel(i + 80)"
            >{{ i + 80 }}</span>
        </div>
    </div>
</template>

<script>
export default {
    name: "SelectCharacterLevel",
    methods: {
        handleClickLevel(key) {
            this.$emit("select", key.toString());
        }
    }
}
</script>

<style scoped>
.panel {
    /* margin-bottom: 16px; */
}

.item {
    cursor: pointer;
    height: 32px;
    width: 32px;
    text-align: center;
    line-height: 32px;
    color: #444444;
    margin-right: 12px;
    margin-bottom: 12px;
    display: inline-block;
    border-radius: 8px;
    /* box-shadow: 0 0 10px 1px rgba(0, 0, 0, 0.1); */
    /* border: 1px solid gray; */
    font-size: 12px;
    transition: 300ms;
}

.item:hover {
    background: rgba(0, 0, 0, 0.1);
}

.special {
    background: #123456;
    color: white;
}

.special:hover {
    background: #345678;
}
</style>