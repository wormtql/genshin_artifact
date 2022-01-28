<template>
    <div>
        <template v-if="list.length > 0">
            <div
                v-for="item in list"
                :key="item.name"
                class="item"
            >
                <span>{{ item.name }}</span>
                <span>{{ item.value.toFixed(3) }}</span>
            </div>
        </template>
        <div class="no-item" v-else style="padding: 8px">
            无加成
        </div>
    </div>
</template>

<script>
export default {
    name: "AttributeComposition",
    props: ["composition"],
    computed: {
        list() {
            let temp = []
            for (const key in this.composition) {
                temp.push({
                    name: key,
                    value: this.composition[key]
                })
            }
            temp.sort((a, b) => a.name.localeCompare(b.name))
            return temp
        }
    }
}
</script>

<style lang="scss" scoped>
.item {
    display: flex;
    justify-content: space-between;
    height: 28px;
    gap: 16px;
    align-items: center;
}
</style>