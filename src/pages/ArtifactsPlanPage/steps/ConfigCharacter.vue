<template>
    <div>
        <div class="config-item">
            <h3 class="config-title">技能等级（包含命之座加成）</h3>
            <el-input-number class="skill" v-model="skill1" :min="1" :max="13"></el-input-number>
            <el-input-number class="skill" v-model="skill2" :min="1" :max="13"></el-input-number>
            <el-input-number class="skill" v-model="skill3" :min="1" :max="13"></el-input-number>
        </div>

        <div class="config-item">
            <h3 class="config-title">命之座</h3>
            <el-input-number v-model="constellation" :min="0" :max="6"></el-input-number>
        </div>

        <div class="config-item">
            <h3 class="config-title">角色等级</h3>
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
// import CustomForm from "../CustomForm";

export default {
    name: "ConfigCharacter",
    components: {
        // CustomForm,
    },
    data: function () {
        return {
            skill1: 6,
            skill2: 6,
            skill3: 6,
            constellation: 0,
        }
    },
    methods: {
        handleClickLevel(key) {
            let s = key.toString();
            let level = parseInt(s);
            let ascend = s.indexOf("+") !== -1;

            let ret = {
                level,
                ascend,
                skill1: this.skill1,
                skill2: this.skill2,
                skill3: this.skill3,
                constellation: this.constellation,
            }

            this.$emit("select", ret);
        }
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

.skill {
    margin-right: 18px;
}
</style>