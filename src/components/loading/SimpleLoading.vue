<template>
    <div class="loading-root">
        <div class="lds-ripple">
            <div></div>
            <div></div>
        </div>
        <p class="text">Loading</p>
    </div>
</template>

<script>
export default {
    name: "SimpleLoading"
}
</script>

<style scoped lang="scss">
.loading-root {
    width: 100%;
    height: 100%;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;

    .text {
        font-size: 12px;
        color: rgb(85,68,127);
    }
}

/* loading animation from https://loading.io/css/ */
.lds-ripple {
    display: inline-block;
    position: relative;
    width: 80px;
    height: 80px;
}
.lds-ripple div {
    position: absolute;
    border: 4px solid rgb(85,68,127);
    opacity: 1;
    border-radius: 50%;
    animation: lds-ripple 1s cubic-bezier(0, 0.2, 0.8, 1) infinite;
}
.lds-ripple div:nth-child(2) {
    animation-delay: -0.5s;
}
@keyframes lds-ripple {
    0% {
        top: 36px;
        left: 36px;
        width: 0;
        height: 0;
        opacity: 1;
    }
    100% {
        top: 0px;
        left: 0px;
        width: 72px;
        height: 72px;
        opacity: 0;
    }
}

</style>