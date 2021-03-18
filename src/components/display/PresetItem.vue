<template>
    <div class="item br-3" @click="$emit('click')">
        <div class="header">
            <span class="fs-12">{{ item.name }}</span>
            <div v-if="toolbar" class="buttons flex-row">
                <el-button
                    icon="el-icon-delete"
                    type="text"
                    size="mini"
                    circle
                    @click="$emit('delete')"
                ></el-button>
            </div>
        </div>
        <div class="body">
            <div class="detail-div fs-12">
                <img :src="characterData.avatar" class="c-avatar br-50p">
                <span>{{ characterData.chs }}</span>
            </div>
            <div class="detail-div fs-12">
                <img :src="weaponData.url" class="w-avatar br-50p">
                <span>{{ weaponData.chs }}</span>
            </div>
            <div class="detail-div fs-12">
                <img :src="tfData.badge" class="tf-avatar br-50p">
                <span>{{ tfData.chs }}</span>
            </div>
        </div>
    </div>
</template>

<script>
import { charactersData } from "@asset/characters";
import { weaponsData } from "@asset/weapons";
import targetFuncsData from "@asset/target_functions/data";

export default {
    name: "PresetItem",
    props: {
        item: {
            type: Object,
            default: () => ({
                name: "123",
                cName: "keqing",
                cArgs: {},
                wName: "heijian",
                wArgs: {},
                tName: "expect",
                tArgs: {},
            })
        },
        toolbar: {
            type: Boolean,
            default: true,
        }
    },
    computed: {
        characterData() {
            return charactersData[this.item.cName];
        },

        weaponData() {
            return weaponsData[this.item.wName];
        },

        tfData() {
            return targetFuncsData[this.item.tName];
        }
    }

}
</script>

<style scoped>
.buttons {
    float: right;
    height: 32px;
}

.item {
    box-shadow: 0 0 10px 1px #00000011;
    display: inline-block;
    transition: 300ms;
}

.header {
    height: 32px;
    border-bottom: 1px solid #00000022;
    /* display: flex; */
    /* justify-content: space-between; */
    /* align-items: center; */
}

.header span {
    line-height: 32px;
    padding: 4px 8px;
    color: #123456;
}

.body {
    padding: 8px;
    display: flex;
}

.c-avatar, .w-avatar, .tf-avatar {
    display: block;
    height: 64px;
    width: 64px;
    border: 1px solid #e9e9e9;
}

.detail-div span {
    width: 64px;
    text-align: center;
    display: block;
    padding-top: 8px;
}

.detail-div {
    margin-right: 12px;
}

.detail-div:last-of-type {
    margin: 0;
}
</style>