<template>
    <div>
        <div class="config-item">
            <h3 class="title">角色等级</h3>
            <div class="panel">
                <span
                    v-for="i in 19"
                    :key="i"
                    class="select-int"
                    @click="handleClickLevel(i)"
                >{{ i }}</span>
                <span class="select-int special" @click="handleClickLevel('20-')">20-</span>
            </div>
            
            <div class="panel">
                <span class="select-int special" @click="handleClickLevel('20+')">20+</span>
                <span
                    v-for="i in 19"
                    :key="i + 20"
                    class="select-int"
                    @click="handleClickLevel(i + 20)"
                >{{ i + 20 }}</span>
                <span class="select-int special" @click="handleClickLevel('40-')">40-</span>
            </div>

            <div class="panel">
                <span class="select-int special" @click="handleClickLevel('40+')">40+</span>
                <span
                    v-for="i in 9"
                    :key="i + 40"
                    class="select-int"
                    @click="handleClickLevel(i + 40)"
                >{{ i + 40 }}</span>
                <span class="select-int special" @click="handleClickLevel('50-')">50-</span>
            </div>

            <div class="panel">
                <span class="select-int special" @click="handleClickLevel('50+')">50+</span>
                <span
                    v-for="i in 9"
                    :key="i + 50"
                    class="select-int"
                    @click="handleClickLevel(i + 50)"
                >{{ i + 50 }}</span>
                <span class="select-int special" @click="handleClickLevel('60-')">60-</span>
            </div>

            <div class="panel">
                <span class="select-int special" @click="handleClickLevel('60+')">60+</span>
                <span
                    v-for="i in 9"
                    :key="i + 60"
                    class="select-int"
                    @click="handleClickLevel(i + 60)"
                >{{ i + 60 }}</span>
                <span class="select-int special" @click="handleClickLevel('70-')">70-</span>
            </div>

            <div class="panel">
                <span class="select-int special" @click="handleClickLevel('70+')">70+</span>
                <span
                    v-for="i in 9"
                    :key="i + 70"
                    class="select-int"
                    @click="handleClickLevel(i + 70)"
                >{{ i + 70 }}</span>
                <span class="select-int special" @click="handleClickLevel('80-')">80-</span>
            </div>

            <div class="panel">
                <span class="select-int special" @click="handleClickLevel('80+')">80+</span>
                <span
                    v-for="i in 10"
                    :key="i + 80"
                    class="select-int"
                    @click="handleClickLevel(i + 80)"
                >{{ i + 80 }}</span>
            </div>
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
.config-item {
    /* border-left: 5px solid #a0cfff; */
    /* padding-left: 20px; */
    padding: 18px;
    margin-bottom: 20px;
    box-shadow: 0 0 20px 1px rgba(0, 0, 0, 0.1);
    border-radius: 3px;
}

.title {
    font-size: 16px;
    color: #555555;
    margin: 0;
    margin-bottom: 12px;
}

.special {
    background: #123456;
    color: white;
}

.special:hover {
    background: #345678;
}
</style>