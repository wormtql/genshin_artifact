<template>
    <div>
        <el-dialog
            :visible.sync="showDialog"
            title="伤害构成"
            width="80%"
        >
            <damage-analysis
                ref="damageAnalysis"
            ></damage-analysis>
        </el-dialog>

        <div
            v-for="item in myList"
            :key="item.name"
            class="item"
            @click="handleClickItem(item.name)"
        >
            <span class="name">{{ item.name }}</span>
            <div class="numbers">
                <span :class="[item.normalClass]" class="number">{{ item.normal }}</span>
                <span v-if="item.showMelt" class="melt number">{{ item.melt }}</span>
                <span v-if="item.showVaporize" class="vaporize number">{{ item.vaporize }}</span>
            </div>
        </div>
    </div>
</template>

<script>
import DamageAnalysis from "@c/display/DamageAnalysis"

export default {
    name: "DamageList",
    components: {
        DamageAnalysis
    },
    props: {
        damageList: {}
    },
    data() {
        return {
            showDialog: false
        }
    },
    methods: {
        handleClickItem(name) {
            this.showDialog = true
            const analysis = this.damageList[name]

            this.$nextTick(() => {
                const component = this.$refs["damageAnalysis"]
                component.setValue(analysis)
            })
        }
    },
    computed: {
        element() {
            return this.damageList
        },

        myList() {
            let temp = []
            for (let key in this.damageList) {
                const value = this.damageList[key]
                const melt = value.element === "Pyro" || value.element === "Cryo"
                const vaporize = value.element === "Pyro" || value.element === "Hydro"

                temp.push({
                    name: key,
                    normal: Math.round(value.normal.expectation),
                    normalClass: value.element.toLowerCase(),
                    showMelt: melt,
                    melt: Math.round(value.melt.expectation),
                    showVaporize: vaporize,
                    vaporize: Math.round(value.vaporize.expectation)
                })
            }
            temp.sort((a, b) => a.name.localeCompare(b.name))
            return temp
        }
    }
}
</script>

<style scoped lang="scss">
.item {
    height: 32px;
    display: flex;
    justify-content: space-between;
    align-items: center;
    font-size: 14px;

    &:hover {
        background-color: rgb(241, 241, 241);
    }

    .name {
        
    }

    .numbers {
        display: flex;
        gap: 4px;
    }

    .number {
        padding: 4px;
        border-radius: 3px;
    }

    .melt {
        color: rgb(63, 63, 63);
        // background-color: rgb(155, 218, 255);
        background-image: url("../../assets/badges/cryo.png");
        // background-size: 48px;
        background-position-x: -20px;
        background-position-y: -30px;
        background-repeat: no-repeat;
    }

    .pyro {
        color: rgb(255, 95, 95);
        background-color: rgb(255, 224, 224);
    }

    .physical {
        color: rgb(71, 71, 71);
        background-color: rgb(218, 218, 218);
    }
}
</style>