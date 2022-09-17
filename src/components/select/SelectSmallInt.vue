<template>
    <div @click="handleClickLevel">
        <span
            v-for="i in (max - min + 1)"
            :key="i"
            class="select-small-int"
            :class="{ active: i + min - 1 === modelValue }"
        >{{ i + min - 1 }}</span>
    </div>
</template>

<script setup lang="ts">
const props = defineProps<{
    modelValue: number
    min: number
    max: number
}>()

const emits = defineEmits<{
    (e: 'update:modelValue', value: number): void
}>()

function handleClickLevel(e: MouseEvent) {
    const target = e.target as any
    if (!target.classList.contains("select-small-int")) {
        return
    }
    emits('update:modelValue', Number(target.textContent))
}
</script>

<style lang="scss" scoped>
.select-small-int {
    cursor: pointer;
    height: 32px;
    width: 32px;
    text-align: center;
    line-height: 32px;
    color: #444444;
    margin-right: 12px;
    /* margin-top: 12px; */
    display: inline-block;
    border-radius: 8px;
    font-size: 12px;
    transition: 300ms;
}
.select-small-int:hover {
    background: rgba(0, 0, 0, 0.1);
}
.select-small-int.active {
    background: #79bbff;
    color: white;
}
/* .active {
    background: #12345622;
} */
</style>
