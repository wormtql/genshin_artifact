<template>
    <ul>
        <li
            v-for="item in items"
            :key="item.index"
            :class="{ active: item.index === value }"
            @click="$emit('input', item.index)"
        >
            {{ item.title }}
        </li>
    </ul>
</template>

<script>
export default {
    name: "MyList",
    props: ["value", "items"]
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

    &.active {
        color: #409EFF;
    }

    &:focus, &:hover {
        background-color: #ecf5ff;
    }
}
</style>