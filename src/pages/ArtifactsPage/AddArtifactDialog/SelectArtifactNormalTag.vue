<template>
    <div>
        <div
            v-for="index in 4"
            :key="index"
            class="item"
        >
            <el-input
                :model-value="displayedValue[index - 1]"
                @update:modelValue="handleValueChange(index - 1, $event)"
                class="input"
            >
                <template #prepend>
                    <el-select
                        :model-value="names[index - 1]"
                        @update:modelValue="handleTagChange(index - 1, $event)"
                        class="prepend"
                    >
                        <el-option
                            v-for="tagData in secondaryTagData"
                            :key="tagData.name"
                            :label="tagData.chs"
                            :value="tagData.name"
                        >
                        </el-option>
                    </el-select>
                </template>

                <template v-if="isPercentage[index - 1]" #append>
                    <span>%</span>
                </template>
            </el-input>

            <el-button
                :icon="IconEpDelete"
                circle
                text
                class="del-button"
                @click="deleteItem(index - 1)"
            ></el-button>
        </div>
    </div>
</template>

<script setup lang="ts">
import { deepCopy } from "@/utils/common"
import { subStats, artifactTags } from "@/constants/artifact"
import type {ArtifactSubStatName} from "@/types/artifact"
import IconEpDelete from "~icons/ep/delete"

const secondaryTagData = subStats.map(item => artifactTags[item])
Object.freeze(secondaryTagData)

interface StringStat {
    name: ArtifactSubStatName | null,
    value: string
}

type Entry = StringStat | null

interface Props {
    modelValue: Entry[]
}

const props = defineProps<Props>()

interface Emits {
    (e: "update:modelValue", v: Entry[]): void
}

const emits = defineEmits<Emits>()

function handleValueChange(index: number, value: string) {
    if (!props.modelValue[index]) {
        return
    }

    let temp = deepCopy(props.modelValue);
    temp[index].value = value;

    emits("update:modelValue", temp)
}

function handleTagChange(index: number, value: ArtifactSubStatName) {
    let temp = deepCopy(props.modelValue);
    if (props.modelValue[index] && (props.modelValue[index] as any).name) {
        temp[index].name = value;
    } else {
        temp.push({
            name: value,
            value: 0,
        });
    }

    emits("update:modelValue", temp)
}

function deleteItem(index: number) {
    let temp = []
    for (let i = 0; i < props.modelValue.length; i++) {
        if (i !== index) {
            temp.push(deepCopy(props.modelValue[i]))
        }
    }

    emits("update:modelValue", temp)
}

const isPercentage = computed(() => {
    let temp = [];

    for (let i = 0; i < 4; i++) {
        try {
            temp.push(artifactTags[props.modelValue[i]?.name as any].percentage);
        } catch(e) {
            temp.push(false);
        }
    }

    return temp;
})

const displayedValue = computed(() => {
    let temp = [];
    for (let i = 0; i < 4; i++) {
        try {
            // @ts-ignore
            let x = props.modelValue[i].value;
            temp.push(x);
        } catch(e) {
            temp.push("0");
        }
    }

    return temp;
})

const names = computed(() => {
    let temp = [];
    for (let i = 0; i < 4; i++) {
        try {
            // @ts-ignore
            let x = props.modelValue[i].name;
            temp.push(x);
        } catch(e) {
            temp.push("");
        }
    }

    return temp;
})
</script>

<style scoped>
.prepend {
    width: 100px;
    text-overflow: ellipsis;
}

.item {
    display: flex;
    align-items: center;
    margin-bottom: 12px;
}

.del-button {
    padding: 0;
    margin-left: 8px;
}
</style>