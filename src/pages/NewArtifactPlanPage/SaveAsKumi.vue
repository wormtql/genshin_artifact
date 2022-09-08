<template>
    <el-dialog
        :model-value="props.modelValue"
        @update:modelValue="emits('update:modelValue', $event)"
        :title="t('calcPage.saveKumi')"
        :width="deviceIsPC ? '60%' : '90%'"
    >
        <p class="title">{{ t("calcPage.selectDir") }}</p>
        <el-checkbox-group
            v-model="checkList"
            :min="1"
        >
            <el-checkbox
                v-for="dir in kumiStore.dirs.value"
                :key="dir.id"
                :label="dir.id"
                border
                class="dir-item"
            >{{ dir.title }}</el-checkbox>
        </el-checkbox-group>

        <p class="title">{{ t("calcPage.name") }}</p>
        <el-input
            v-model="name"
            :placeholder="t('calcPage.enterName')"
        ></el-input>

        <template #footer>
            <el-button @click="$emit('update:modelValue', false)">{{ t("misc.cancel") }}</el-button>
            <el-button type="primary" :disabled="name === ''" @click="handleConfirm">{{ t("misc.confirm") }}</el-button>
        </template>

<!--        <div class="buttons" style="text-align: right; margin-top: 32px">-->
<!--            <el-button-->
<!--                type="primary"-->
<!--                :disabled="name === ''"-->
<!--                @click="handleConfirm"-->
<!--            >确定</el-button>-->

<!--        </div>-->
    </el-dialog>
</template>

<script setup lang="ts">
import {deviceIsPC} from "@/utils/device"
import {useKumiStore} from "@/store/pinia/kumi"
import {useI18n} from "@/i18n/i18n"

const { t } = useI18n()

const kumiStore = useKumiStore()

interface Props {
    defaultName: string,
    modelValue: boolean,
}

const props = withDefaults(defineProps<Props>(), {
    defaultName: ""
})

interface Emits {
    (e: "confirm", v: { dirIds: number[], name: string }): void,
    (e: "update:modelValue", v: boolean): void
}

const emits = defineEmits<Emits>()

const checkList = ref<number[]>([0])
const name = ref<string>(props.defaultName)

function handleConfirm() {
    const dirIds = checkList.value

    emits("confirm", {
        dirIds, name: name.value
    })
}

watch(() => props.defaultName, newName => {
    name.value = newName
})
</script>

<style scoped>

</style>
