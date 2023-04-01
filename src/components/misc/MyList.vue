<template>
    <ul>
        <li
            v-for="item in items"
            :key="item.index"
            :class="{ active: item.index === modelValue }"
            @click="$emit('update:modelValue', item.index)"
        >
            {{ item.title }}
        </li>
    </ul>
</template>

<script>
export default {
    name: "MyList",
    props: ["modelValue", "items"],
    emits: ["update:modelValue"]
}
</script>

<style scoped lang="scss">
$height: 48px;

ul, li {
    padding: 0;
    margin: 0;
}

li {
    cursor: pointer;
    padding: 0 20px;
    line-height: $height;
    height: $height;
    font-size: 14px;
    list-style: none;
    transition: 200ms;
    white-space: nowrap;

    &.active {
        color: #409EFF;
        background-color: lighten(#ecf5ff, 0.1)
    }

    &:focus, &:hover {
        background-color: #ecf5ff;
    }
}
</style>