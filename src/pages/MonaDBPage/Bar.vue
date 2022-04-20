<template>
    <div class="bar" :style="style">
        {{ (fraction * 100).toFixed(1) }}%
    </div>
</template>

<script>
export default {
    name: "Bar",
    props: ["fraction"],
    computed: {
        style() {
            return {
                // 100% - image width - image right margin - ...
                width: `calc((100% - 64px - 12px) * ${this.fraction})`
            }
        }
    }
}
</script>

<style lang="scss" scoped>
$height: 24px;
.bar {
    font-size: 12px;
    background-color: rgb(236, 245, 255);
    color: #909399;
    padding-left: 12px;
    height: $height;
    border-radius: 3px;
    line-height: $height;

}
</style>