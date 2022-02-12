<template>
    <div class="config-root" :style="styleRoot">
        <ConfigItem
            class="config"
            v-for="config in configs"
            :key="config.name"
            :params="config"
            :type="config.type"
            :value="value[itemName][config.name]"
            @input="handleInput(config.name, $event)"
        ></ConfigItem>
    </div>
</template>

<script>
import ConfigItem from "./ConfigItem"

export default {
    name: "ItemConfig",
    components: {
        ConfigItem
    },
    props: {
        value: {},
        itemName: {},
        configs: {
            type: Array
        },
        bg: {
            default: "rgb(239, 246, 253)"
        }
    },
    computed: {
        styleRoot() {
            return {
                backgroundColor: this.bg
            }
        }
    },
    
    methods: {
        handleInput(name, value) {
            let obj = Object.assign({}, this.value[this.itemName])
            obj[name] = value

            this.$emit("input", {
                [this.itemName]: obj
            })
        }
    }
}
</script>

<style lang="scss" scoped>
.config-root {
    padding: 12px;
    border-radius: 3px;
    // background-color: rgb(239, 246, 253);

    .config {
        margin-bottom: 8px;
        &:last-of-type {
            margin-bottom: 0;
        }
    }
}
</style>
