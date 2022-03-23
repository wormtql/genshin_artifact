<template>
    <div class="root">
        <div class="back" :style="styleForBack"></div>
        <span class="display">{{ (value * 100).toFixed(1) }}
            <span v-if="extra">/ {{ extra }}</span>
        </span>
    </div>
</template>

<script>
export default {
    name: "ValueDisplay",
    props: {
        "value": {},
        "extra": {
            default: ""
        }
    },
    computed: {
        styleForBack() {
            return {
                width: `${this.value * 100}%`
            }
        }
    }
}
</script>

<style scoped lang="scss">
.root {
    position: relative;
    display: flex;
    align-items: center;
    justify-content: center;
    height: 28px;

    .back {
        position: absolute;
        left: 0;
        top: 0;
        height: 100%;
        background-color: rgb(217, 236, 255);
        z-index: -1;
    }

    .display {
        color: black;
        font-size: 12px;
    }
}
</style>