<template>
    <div>
        <div
            class="root"
            :class="{ over: isDragover }"
            ref="rootBox"
            @dragenter="handleDragEnter"
            @dragleave="handleDragLeave"
            @dragover="handleDragover"
            @drop="handleDrop"
            @click="handleClick"
        >
            <div class="center-div">
                <el-icon>
                    <i-fa6-solid-upload></i-fa6-solid-upload>
                </el-icon>
<!--                <i class="el-icon-upload"></i>-->
                <span>点击，或拖拽上传</span>
            </div>
        </div>
        <p v-if="hasFile" class="filename">{{ fileName }}</p>

        <input type="file" ref="fileInput" class="file-input"
            @change="handleSelectFile"
            :accept="props.accept"
        />
    </div>

</template>

<script setup lang="ts">
// drag drop
import {Ref} from "vue";

const props = defineProps<{
    accept?: string
}>()

const isDragover = ref(false)

function handleDragEnter(e: DragEvent) {
    e.preventDefault()

    isDragover.value = true
}

function handleDragLeave(e: DragEvent) {
    e.preventDefault()

    isDragover.value = false
}

function handleDragover(e: DragEvent) {
    e.preventDefault()
}


// select or drop file
const file: Ref<null | File> = ref(null)
const hasFile = ref(false)
const fileName = ref("")
const fileInput: Ref<null | HTMLInputElement> = ref(null)

function handleClick() {
    if (!fileInput.value) {
        return
    }

    fileInput.value.click()
}

function handleSelectFile() {
    if (!fileInput.value) {
        return
    }

    if (fileInput.value.files) {
        const files = Array.from(fileInput.value.files)
        // console.log(files)
        if (files.length > 0) {
            file.value = files[0]
            hasFile.value = true
            fileName.value = files[0].name
        }
    }
}

function handleDrop(ev: DragEvent) {
    ev.preventDefault()

    isDragover.value = false

    let f: File | null = null

    if (ev.dataTransfer) {
        if (ev.dataTransfer.items) {
            // Use DataTransferItemList interface to access the file(s)
            for (let i = 0; i < ev.dataTransfer.items.length; i++) {
                // If dropped items aren't files, reject them
                if (ev.dataTransfer.items[i].kind === 'file') {
                    f = ev.dataTransfer.items[i].getAsFile()
                    break
                    // console.log('... file[' + i + '].name = ' + file.name);
                }
            }
        } else {
            // Use DataTransfer interface to access the file(s)
            f = ev.dataTransfer.files[0]
        }
    }


    // if (!file) {
    //     return
    // }

    // let fileReader = new FileReader()
    file.value = f

    if (f) {
        hasFile.value = true
        fileName.value = f.name
    }

    // let df = e.dataTransfer
    // if (df.length === 0) {
    //     return;
    // }
    //
    // let file = df.files[0];
    // let fileReader = new FileReader();
    //
    // fileReader.onload = (readResult) => {
    //     this.json = readResult.target.result;
    // };
    // fileReader.readAsText(file);
}

function getReadPromise(): Promise<string> {
    return new Promise((resolve, reject) => {
        if (!file.value) {
            reject("未选择文件")
        }

        try {
            let fileReader = new FileReader()

            fileReader.onload = result => {
                if (result.target) {
                    resolve(result.target.result as string)
                }
            }
            fileReader.onerror = error => {
                reject(error)
            }

            fileReader.readAsText(file.value as File)
        } catch(e) {
            reject(e)
        }
    })
}

defineExpose({
    getReadPromise
})
</script>

<style scoped lang="scss">
.root {
    border: 1px dashed #d9d9d9;
    height: 180px;
    border-radius: 5px;
    display: flex;
    //flex-direction: column;
    align-items: center;
    justify-content: center;

    &:hover {
        border: 1px dashed #409eff;
    }

    &.over {
        border: 1px dashed #409eff;
        background-color: rgba(32, 159, 255, .06);
    }

    .center-div {
        //text-align: center;
        display: flex;
        flex-direction: column;
        align-items: center;
        //justify-content: center;
        pointer-events: none;

        i {
            font-size: 30px;
            color: #c0c4cc;
        }

        span {
            font-size: 14px;
            color: #606266;
            margin-top: 8px;
        }
    }
}

.filename {
    font-size: 12px;
    color: #606266;
    margin: 0;
    height: 28px;
    line-height: 28px;
}

.file-input {
    display: none;
}
</style>
