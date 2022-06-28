<template>
    <el-card shadow="never">
        <template #header>
            {{ props.item.name }}
        </template>

        <div class="words" v-if="props.item.description">
            <span class="quote">“</span>
            <span class="content">{{ props.item.description }}</span>
        </div>

        <div class="link-tags">
            <span
                class="link-tag fs-12"
                v-for="(tag, index) in props.item.tags"
                :key="index"
            >{{ tag }}</span>
        </div>

        <div class="link-footer">
            <el-button-group>
                <el-button
                    v-for="(item, index) in props.item.url"
                    :key="index"
                    :icon="item.icon"
                    :type="item.primary ? 'primary' : undefined"
                >{{ item.title }}</el-button>
            </el-button-group>
        </div>
    </el-card>
</template>

<script setup lang="ts">
import type { LinkItem } from "./types"

interface Props {
    item: LinkItem
}

const props = defineProps<Props>()
</script>

<style scoped lang="scss">
.words {
    .quote {
        font-size: 3rem;
        padding: 0 8px;
        vertical-align: baseline;
        position: relative;
        line-height: 16px;
        top: 18px;
    }

    .content {
        font-size: 14px;
        vertical-align: bottom;
    }

    background: #f4f4f5;
    color: #909399;
    padding: 16px;
    border-radius: 3px;
    margin-bottom: 8px;
}

.link-tag {
    background: #409eff;
    color: white;
    height: 24px;
    border-radius: 12px;
    padding: 0 8px;
    line-height: 24px;
    margin: 8px 8px 0 0;
}

.link-tags {
    display: flex;
    flex-wrap: wrap;
}

.link-footer {
    margin-top: 24px;
}
</style>