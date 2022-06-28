<template>
    <div class="root"
        @click="handleClick"
    >
        <div class="left">
<!--            <div class="icon">-->
                <el-icon :size="30"><component :is="icon"></component></el-icon>
<!--            </div>-->
        </div>

        <div class="right">
            <p class="text">{{ text }}</p>
            <p class="description" v-html="description"></p>
        </div>

        <div class="back">
            <slot>
                <div class="center">
                    <el-icon :size="30"><i-fa6-solid-link></i-fa6-solid-link></el-icon>
                    <!--                立刻开启-->
                </div>
            </slot>


        </div>
    </div>
</template>

<script>
// import IconFa6SolidLink from "~icons/fa6-solid/link"

export default {
    name: "UseCaseItem",
    props: ["text", "url", "icon", "description"],
    emits: ["click"],
    methods: {
        handleClick() {
            // console.log(this.url)
            if (this.url) {
                this.$router.replace(this.url)
            } else {
                this.$emit("click")
            }
        }
    }
}
</script>

<style lang="scss" scoped>
$mona-color: rgb(86,72,132);

.description {
    font-size: 14px;
    color: #606266;
    margin-top: 4px;
    margin-bottom: 0;
}

.text {
    font-size: 18px;
    margin: 0;
    font-weight: bold;
    //color: $mona-color;
    color: var(--el-color-primary);
}

.left {
    padding-right: 20px;
}

.root {
    position: relative;
    //text-align: center;
    border: 1px solid #00000011;
    //height: 100px;
    height: 100%;
    box-sizing: border-box;
    overflow: hidden;
    display: flex;
    //flex-direction: column;
    //align-items: center;
    //justify-content: center;
    cursor: pointer;
    padding: 20px;

    .back {
        position: absolute;
        top: 0;
        left: 0;
        bottom: 0;
        right: 0;
        background-color: #00000033;
        //opacity: 0;

        display: flex;
        align-items: center;
        justify-content: center;
        transform: translateY(100%);
        transition: 100ms;

        //color: white;
        font-size: 16px;

        .center {
            background-color: white;
            display: flex;
            align-items: center;
            justify-content: center;
            height: 48px;
            width: 48px;
            border-radius: 4px;
        }
    }


    &:hover {
        //background-color: rgb(251, 249, 255);

        .back {
            //opacity: 1;
            transform: none;
        }
    }
}


</style>