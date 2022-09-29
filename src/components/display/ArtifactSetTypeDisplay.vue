<template>
    <div class="root">
        <template v-if="isMisc">散件</template>
        <template v-else-if="isSet2">
            <img :src="images[0]" class="image">×2
        </template>
        <template v-else-if="isSet4">
            <img :src="images[0]" class="image">×4
        </template>
        <template v-else-if="isSet22">
            <img :src="images[0]" class="image">
            <img :src="images[1]" class="image">
        </template>
    </div>
</template>

<script setup lang="ts">
import { convertArtifactNameBack } from "@/utils/converter"
import { getArtifactThumbnail } from "@/utils/artifacts"

type SetType = { Set2: string } | { Set4: string } | { Set22: string[] } | "Misc"

interface Props {
    setType: SetType,
}
const props = defineProps<Props>()

const images = computed(() => {
    let names = []

    if (typeof(props.setType) === "string") {
        return []
    }

    if ("Set2" in props.setType) {
        names.push(props.setType["Set2"])
    } else if ("Set4" in props.setType) {
        names.push(props.setType["Set4"])
    } else if ("Set22" in props.setType) {
        names.push(props.setType["Set22"][0])
        names.push(props.setType["Set22"][1])
    }

    let temp = []
    for (let name of names) {
        let convertedName = convertArtifactNameBack(name)
        // const data = artifactsData[convertedName]
        const url = getArtifactThumbnail(convertedName)
        temp.push(url)
    }

    return temp
})

const isSet2 = computed(() => {
    return typeof(props.setType) !== "string" && "Set2" in props.setType
})

const isSet4 = computed(() => {
    return typeof(props.setType) !== "string" && "Set4" in props.setType
})

const isSet22 = computed(() => {
    return typeof(props.setType) !== "string" && "Set22" in props.setType
})

const isMisc = computed(() => {
    return props.setType === "Misc"
})

</script>

<style scoped lang="scss">
.image {
    width: 48px;
    height: 48px;
    border-radius: 50%;
    margin-right: 8px;
}

.root {
    display: flex;
    align-items: center;
}
</style>