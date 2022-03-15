<template>
    <div>
        <div
            class="root"
            :class="{ over: isDragOver }"
            ref="rootBox"
            @dragenter="handleDragEnter"
            @dragleave="handleDragLeave"
            @dragover="handleDragover"
            @drop="handleDrop"
            @click="handleClick"
        >
            <div class="center-div">
                <i class="el-icon-upload"></i>
                <span>点击，或拖拽上传</span>
            </div>
        </div>
        <p v-if="hasFile" class="filename">{{ fileName }}</p>

        <input type="file" ref="fileInput" class="file-input"
            @change="handleSelectFile"
        />
    </div>

</template>

<script>
export default {
    name: "ImportBlock",
    mounted() {
        // const ele = this.$refs.rootBox
        // console.log(!!window.FileReader)
        // ele.ondrop = e => {
        //     e.preventDefault()
        //     this.handleDrop(e)
        // }
        this.file = null
    },
    data() {
        return {
            isDragOver: false,

            hasFile: false,
            fileName: "",
        }
    },
    methods: {
        handleDrop(ev) {
            ev.preventDefault()

            this.isDragOver = false
            // console.log(ev)

            let file = null
            if (ev.dataTransfer.items) {
                // Use DataTransferItemList interface to access the file(s)
                for (let i = 0; i < ev.dataTransfer.items.length; i++) {
                    // If dropped items aren't files, reject them
                    if (ev.dataTransfer.items[i].kind === 'file') {
                        file = ev.dataTransfer.items[i].getAsFile()
                        break
                        // console.log('... file[' + i + '].name = ' + file.name);
                    }
                }
            } else {
                // Use DataTransfer interface to access the file(s)
                file = ev.dataTransfer.files[0].name
            }

            // if (!file) {
            //     return
            // }

            // let fileReader = new FileReader()
            this.file = file

            if (file) {
                this.hasFile = true
                this.fileName = file.name
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
        },

        getReadPromise() {
            return new Promise((resolve, reject) => {
                if (!this.file) {
                    reject("未选择文件")
                }

                try {
                    let fileReader = new FileReader()

                    fileReader.onload = result => {
                        resolve(result.target.result)
                    }
                    fileReader.onerror = error => {
                        reject(error)
                    }

                    fileReader.readAsText(this.file)
                } catch(e) {
                    reject(e)
                }
            })
        },

        handleDragEnter(e) {
            e.preventDefault()

            this.isDragOver = true
        },

        handleDragLeave(e) {
            e.preventDefault()

            this.isDragOver = false
        },

        handleDragover(e) {
            e.preventDefault()
            // console.log(e)
        },

        handleClick() {
            const component = this.$refs["fileInput"]
            if (!component) {
                return
            }

            component.click()
        },

        handleSelectFile(e) {
            // console.log(e)
            const component = this.$refs["fileInput"]
            if (!component) {
                return
            }
            const files = Array.from(component.files)
            // console.log(files)
            if (files.length > 0) {
                this.file = files[0]
                this.hasFile = true
                this.fileName = files[0].name
            }
        }
    }
}
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
            font-size: 67px;
            color: #c0c4cc;
        }

        span {
            font-size: 14px;
            color: #606266;
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
