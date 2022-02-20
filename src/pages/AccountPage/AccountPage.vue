<template>
    <div>
        <el-breadcrumb>
            <el-breadcrumb-item>账号</el-breadcrumb-item>
        </el-breadcrumb>
        <el-divider></el-divider>

        <div class="toolbar">
            <el-button
                type="primary"
                icon="el-icon-plus"
                size="mini"
                @click="addAccount"
            >
                添加账号
            </el-button>
        </div>

        <div class="body">
            <div
                v-for="{id, name} in allAccounts"
                :key="id"
                class="item is-active"
                :class="{ active: id === currentAccountId }"
                @click="changeAccount(id)"
            >
                <click-edit-label
                    :value="name"
                    @input="newName => handleChangeName(id, name, newName)"
                    fontsize="16px"
                    :editable="true"
                    style="display: inline-block;"
                ></click-edit-label>
                <div class="buttons flex-row">
                    <el-popconfirm
                        v-show="id !== currentAccountId"
                        title="确定删除？"
                        @confirm="deleteAccount(id)"
                    >
                        <el-button
                            slot="reference"
                            icon="el-icon-delete"
                            type="text"
                            size="medium"
                            circle
                            class="button"
                            title="删除"
                            @click.stop=""
                        ></el-button>
                    </el-popconfirm>
                    <!-- <el-button
                        icon="el-icon-download"
                        type="text"
                        size="medium"
                        circle
                        @click="handleDownload(name, item)"
                        class="button"
                        title="导出"
                    ></el-button> -->
                </div>
            </div>
        </div>
    </div>
</template>

<script>
import { mapState } from "vuex";

import ClickEditLabel from "@c/misc/ClickEditLabel";

export default {
    name: "AccountPage",
    components: {
        ClickEditLabel,
    },
    created() {
        this.canCopy = !!navigator.clipboard;
    },
    computed: {
        ...mapState("accounts", ["currentAccountId", "allAccounts"]),
    },
    methods: {
        addAccount() {
            this.$store.commit('accounts/addAccount', { name: '新账户' });
        },
        deleteAccount(id) {
            this.$store.commit('accounts/deleteAccount', { id });
        },
        changeAccount(id) {
            if (id !== this.currentAccountId) {
                this.$store.dispatch('changeAccount', { id });
            }
        },
        handleChangeName(id, oldName, newName) {
            if (newName !== oldName && newName !== '') {
                this.$store.commit('accounts/changeAccountName', { id, name: newName });
            }
        },
    },
}
</script>

<style lang="scss" scoped>
.body {
    display: flex;
    flex-wrap: wrap;
}

.toolbar {
    margin-bottom: 20px;
}

$height: 60px;

.item {
    height: $height;
    width: 100%;
    padding-left: 20px;
    cursor: pointer;
    line-height: $height;
    color: #303133;
    transition: 200ms;

    &.active {
        color: #409EFF;
        background: #ecf5ff;
    }

    &:hover {
        background: #ecf5ff;
    }

    .buttons {
        float: right;
        height: $height;

        .button {
            margin: 0;
        }
    }
}
</style>
