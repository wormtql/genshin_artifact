<template>
    <div class="root">
        <input
            :value="value"
            @click="edit = true"
            @blur="onBlur"
            :style="inputStyle"
            :disabled="!editable"
        >
        <span class="border"></span>
    </div>
</template>

<script>
export default {
    name: "ClickEditLabel",
    props: {
        "value": {},
        "fontsize": {},
        editable: { default: true }
    },
    data() {
        return {
            edit: false,
        }
    },
    methods: {
        onBlur(e) {
            let text = e.target.value;
            this.edit = false;
            this.$emit("input", text);
        }
    },
    computed: {
        inputStyle() {
            let fontsize = this.fontsize ?? "14px";

            let temp = {
                fontSize: fontsize,
            }

            // if (this.edit) {
            //     Object.assign(temp, {
            //         background: "#00000008",
            //     })
            // }

            return temp;
        }
    }
}
</script>

<style lang="scss" scoped>
.root {
    position: relative;
    // display: inline-block;

    .border {
        position: absolute;
        bottom: 0;
        height: 2px;
        background: #409EFF;
        // width: 100%;
        width: 0;
        // transform: scaleX(0);
        transform-origin: center;
        transition: 300ms;
    }

    input {
        border: none;
        outline: none;
        padding: 8px;
        display: block;
        width: 100%;
        box-sizing: border-box;
        background: none;

        &:focus {
            // background: #00000008;

            + .border {
                // transform: scaleX(1);
                width: 100%;
            }
        }
    }
}


</style>