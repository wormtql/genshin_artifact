<template>
    <div>
        <div class="buff-config-item">
            <p class="buff-config-title">精炼等级</p>
            <el-input-number
                v-model="refine"
                :min="1"
                :max="5"
                size="mini"
            ></el-input-number>
        </div>
    </div>
</template>

<script>
export default {
    name: "Taolong.buffcfg",
    data: function () {
        return {
            refine: 1,
        }
    },
    methods: {
        getValue() {
            return this.refine * 0.06 + 0.18;
        },

        getStandardConfig() {
            return {
                type: "atk-percentage",
                value: this.getValue(),
            }
        },

        getBuff() {
            return {
                name: "taolong",
                args: {
                    refine: this.refine
                }
            }
        },

        setBuff(buff) {
            this.refine = buff.refine;
        }
    }
}
</script>