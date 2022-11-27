<template>
    <div class="root">
        <el-menu
            :default-active="doRoute ? $route.fullPath : '/intro'"
            style="border: none"
            @select="doRoute ? undefined : handleSelect($event)"
            :mode="mode"
            :router="doRoute"
        >
            <el-menu-item index="/intro">
                <el-icon><i-ep-home-filled></i-ep-home-filled></el-icon>
                <span>{{ t("nav.home") }}</span>
            </el-menu-item>
            <el-menu-item index="/setup">
                <el-icon><i-ep-setting></i-ep-setting></el-icon>
                <span>{{ t("nav.setup") }}</span>
            </el-menu-item>
            <el-menu-item index="/account">
                <el-icon><i-ep-user /></el-icon>
                {{ t("nav.account") }} ( {{ currentAccountName }} )
                <div class="sync-icon">
                    <el-icon>
                        <i-ep-folder-checked v-if="syncStatus === 'synced'" />
                        <i-ep-sort v-else-if="syncStatus === 'syncing'" />
                        <i-ep-folder-remove v-else/>
                    </el-icon>
                </div>
            </el-menu-item>

            <el-menu-item-group>
                <template #title>
                    {{ t("nav.repo") }}
                </template>
                <el-menu-item index="/artifacts">
                    <el-icon><i-ep-help-filled /></el-icon>
                    {{ t("nav.artifact") }}
                </el-menu-item>
                <el-menu-item index="/artifacts-kumi">
                    <el-icon><i-ep-help-filled /></el-icon>
                    {{ t("nav.kumi") }}
                </el-menu-item>
                <el-menu-item index="/presets">
                    <el-icon><i-ep-menu /></el-icon>
                    {{ t("nav.preset") }}
                </el-menu-item>
            </el-menu-item-group>

            <el-menu-item-group>
                <template #title>
                    {{ t("nav.compute") }}
                </template>
                <el-menu-item index="/calculate">
                    <el-icon><i-ep-cpu /></el-icon>
                    {{ t("nav.calculate") }}
                </el-menu-item>
                <el-menu-item index="/sequential-optimization">
                    <el-icon><i-ep-cpu /></el-icon>
                    {{ t("nav.sequenceOptimize") }}
                </el-menu-item>
                <el-menu-item index="/team-optimization">
                    <el-icon><i-ep-cpu /></el-icon>
                    {{ t("nav.teamOptimize") }}
                </el-menu-item>
                <el-menu-item index="/potential">
                    <el-icon><i-ep-opportunity /></el-icon>
                    {{ t("nav.potential") }}
                </el-menu-item>
                <el-menu-item index="/best-set">
                    <el-icon><i-ep-medal /></el-icon>
                    {{ t("nav.bestSet") }}
                </el-menu-item>
                <el-menu-item index="/character">
                    <el-icon><i-ep-histogram /></el-icon>
                    {{ t("nav.monaDB") }}
                </el-menu-item>
            </el-menu-item-group>

            <el-menu-item-group>
                <template #title>{{ t("nav.other") }}</template>
                <el-menu-item index="/playground">
                    <el-icon><i-fa6-solid-terminal /></el-icon>
                    {{ t("nav.playground") }}
                </el-menu-item>
            </el-menu-item-group>

            <el-menu-item-group>
                <template #title>
                    {{ t("nav.about") }}
                </template>
                <el-sub-menu index="help">
                    <template #title>
                        <el-icon><i-ep-question-filled /></el-icon>
                        <i class="el-icon-question"></i>
                        {{ t("nav.help") }}
                    </template>
                    <el-menu-item index="/help/export-tools">
                        <el-icon><i-ep-aim /></el-icon>
                        {{ t("nav.exportTool") }}
                    </el-menu-item>
                </el-sub-menu>

                <el-menu-item index="/tomodachi">
                    <el-icon><i-ep-link /></el-icon>
                    {{ t("nav.link") }}
                </el-menu-item>
            </el-menu-item-group>
        </el-menu>
    </div>
</template>

<script lang="ts">
import { defineComponent } from "vue"
import {useI18n} from "@/i18n/i18n"
import { useAccountStore } from "@/store/pinia/account"

export default defineComponent({
    name: "SideBar",
    props: {
        items: {
            type: Array,
            default: () => [],
        },
        mode: {
            type: String,
            default: "vertical",
        },
        doRoute: {
            type: Boolean,
            default: true,
        },
        enableMinimize: {
            type: Boolean,
            default: false
        }
    },
    data() {
        return {
            minimized: false,
        }
    },
    methods: {
        handleSelect(index: string) {
            this.$emit("select", index)
        }
    },
    setup() {
        const { t } = useI18n()
        const accountStore = useAccountStore()

        return {
            t,
            syncStatus: accountStore.syncStatus,
            currentAccountName: accountStore.currentAccountName,
        }
    }
})
</script>

<style scoped>

.item {
    padding: 0 32px;
}

.sync-icon {
    width: 100%;
    text-align: right;
}
</style>
