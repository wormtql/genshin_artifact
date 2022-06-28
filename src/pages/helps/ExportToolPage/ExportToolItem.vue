<template>
    <el-card shadow="never">
        <template #header>
            {{ props.name }}
        </template>

        <span class="body">
            {{ props.description }}
        </span>


        <div>
            <el-button-group>
                <el-button
                    v-for="(item, index) in props.buttons"
                    :key="index"
                    :icon="item.icon"
                    :type="item.primary ? 'primary' : undefined"
                    @click="navigate(item.link)"
                >{{ item.label }}</el-button>
            </el-button-group>
        </div>
    </el-card>
</template>

<script setup lang="ts">
interface Props {
    name: string,
    description: string,
    buttons: {
        label: string,
        primary: boolean,
        link: string,
        icon: any
    }[]
}

const props = defineProps<Props>()

function navigate(link: string) {
    window.open(link, "_blank")
}

</script>

<style scoped lang="scss">
.body {
    display: block;
    font-size: 14px;
    margin-bottom: 20px;
}
</style>