<template>
    <div>
        <span class="dmg-crit" v-if="showCritical">
            {{ convert(damage.crit) }}
        </span>
        <span class="dmg-non-crit" v-if="showNonCritical">
            {{ convert(damage.nonCrit) }}
        </span>
        <span class="dmg-expect" v-if="showExpect">
            {{ convert(damage.expect) }}
        </span>
    </div>
</template>

<script>
export default {
    name: "DamageDisplay",
    props: {
        damage: {
            type: Object,
        },
        showExpect: { type: Boolean, default: true },
        showCritical: { type: Boolean, default: true },
        showNonCritical: { type: Boolean, default: true },
    },
    methods: {
        convert(value) {
            if (typeof value === "string") {
                return value;
            } else {
                return value.toFixed(1);
            }
        }
    }
}
</script>

<style scoped>
.dmg-crit, .dmg-expect, .dmg-non-crit {
    /* color: white; */
    /* border-radius: 3px; */
    padding: 0 4px;
    font-size: 12px;
    display: inline-block;
}

.dmg-crit {
    /* background: #409eff; */
    /* border: 1px solid #409eff; */
    color: #409eff;
    margin-right: 4px;
}

.dmg-non-crit {
    /* border: 1px solid #E6A23C; */
    color: #E6A23C;
    margin-right: 4px;
}

.dmg-expect {
    /* border: 1px solid #67c23a; */
    color: #67c23a;
}
</style>