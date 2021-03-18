<template>
    <div class="config-item">
        <h3 class="config-title">{{ title }}</h3>

        <div class="delegate" @click="handleClickLevel">
            <div class="panel">
                <span
                    v-for="i in 19"
                    :key="i"
                    class="select-int"
                    :class="{ active: i.toString() === value }"
                >{{ i }}</span>
                <span class="select-int" :class="{ active: value === '20-' }">20-</span>
            </div>
            
            <div class="panel">
                <span class="select-int" :class="{ active: value === '20+' }">20+</span>
                <span
                    v-for="i in 19"
                    :key="i + 20"
                    class="select-int"
                    :class="{ active: (i + 20).toString() === value }"
                >{{ i + 20 }}</span>
                <span class="select-int" :class="{ active: value === '40-' }">40-</span>
            </div>

            <div class="panel">
                <span class="select-int" :class="{ active: value === '40+' }">40+</span>
                <span
                    v-for="i in 9"
                    :key="i + 40"
                    class="select-int"
                    :class="{ active: (i + 40).toString() === value }"
                >{{ i + 40 }}</span>
                <span class="select-int" :class="{ active: value === '50-' }">50-</span>
            </div>

            <div class="panel">
                <span class="select-int" :class="{ active: value === '50+' }">50+</span>
                <span
                    v-for="i in 9"
                    :key="i + 50"
                    class="select-int"
                    :class="{ active: (i + 50).toString() === value }"
                >{{ i + 50 }}</span>
                <span class="select-int" :class="{ active: value === '60-' }">60-</span>
            </div>

            <div class="panel">
                <span class="select-int" :class="{ active: value === '60+' }">60+</span>
                <span
                    v-for="i in 9"
                    :key="i + 60"
                    class="select-int"
                    :class="{ active: (i + 60).toString() === value }"
                >{{ i + 60 }}</span>
                <span class="select-int" :class="{ active: value === '70-' }" v-if="star >= 3">70-</span>
                <span class="select-int" :class="{ active: value === '70' }" v-else>70</span>
            </div>

            <div class="panel" v-if="star >= 3">
                <span class="select-int" :class="{ active: value === '70+' }">70+</span>
                <span
                    v-for="i in 9"
                    :key="i + 70"
                    class="select-int"
                    :class="{ active: i + 70 == value }"
                >{{ i + 70 }}</span>
                <span class="select-int" :class="{ active: value === '80-' }">80-</span>
            </div>

            <div class="panel" v-if="star >= 3">
                <span class="select-int" :class="{ active: value === '80+' }">80+</span>
                <span
                    v-for="i in 10"
                    :key="i + 80"
                    class="select-int"
                    :class="{ active: i + 80 == value }"
                >{{ i + 80 }}</span>
            </div>
        </div>
    </div>
</template>

<script>
export default {
    name: "SelectLevel",
    props: {
        value: {
            type: String,
        },

        star: {
            type: Number,
            default: 5,
        }
    },
    methods: {
        handleClickLevel(e) {
            if (!e.target.classList.contains("select-int")) {
                return;
            }
            let s = e.target.textContent;

            this.$emit("input", s);
        }
    },
    computed: {

    }
}
</script>

<style scoped>
.special {
    background: #123456;
    color: white;
}

.special:hover {
    background: #345678;
}

/* .active {
    background: #12345622;
} */
</style>