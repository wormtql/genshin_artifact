<template>
<!--    <div class="root">-->
<!--        <el-popover-->
<!--            placement="right"-->
<!--            trigger="hover"-->
<!--            :visible-arrow="true"-->
<!--        >-->
<!--            <img class="image" :src="data.url" slot="reference" >-->
            <div class="detail">
                <img class="image" :src="data.url" >
                <div class="detail-body">
                    <p class="name">{{ ta(nameLocaleIndex) }}</p>
                    <p class="description" v-if="data.effect" v-html="ta(effectLocaleIndex)"></p>
                </div>

            </div>
<!--        </el-popover>-->
<!--    </div>-->
</template>

<script>
import { weaponData } from "@weapon"
import {useI18n} from "../../i18n/i18n";

export default {
    name: "WeaponDisplay",
    props: {
        weaponName: { },
    },
    computed: {
        data() {
            return weaponData[this.weaponName]
        },

        nameLocaleIndex() {
            return this.data.nameLocale
        },

        effectLocaleIndex() {
            return this.data.effect
        },
    },
    setup() {
        const { t, ta } = useI18n()

        return {
            t,
            ta
        }
    }
}
</script>

<style scoped lang="scss">
$size: 64px;

p {
    font-size: 12px;
    color: #606266;
}

.image {
    width: $size;
    height: $size;
    border-radius: 50%;
}

.detail {
    display: flex;
    //align-items: flex-start;

    .detail-body {
        padding-left: 12px;

        .name {
            margin-top: 0;
            margin-bottom: 12px;
            font-weight: bold;
        }

        .description {
            margin: 0;
            word-break: break-all;
            //width: 400px;
        }
    }
}
</style>