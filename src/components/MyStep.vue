<template>
    <div class="my-step">
        <template
            v-for="(step, index) in steps"
        >
            <div
                :key="'title' + step"
                class="title hand"
                :class="{current: index === pointer, past: index < pointer, future: index > pointer}"
            >
                {{ step }}
            </div>
            <p
                v-if="index !== steps.length - 1"
                :key="'divider' + step"
                class="divider"
            ></p>
        </template>
    </div>
</template>

<script>
export default {
    name: "MyStep",
    props: {
        steps: {
            type: Array,
            default: () => ["1", "2", "3"]
        },
        pointer: {
            type: Number,
            default: 0,
        }
    }
}
</script>

<style scoped>
.my-step {
    display: flex;
    background: rgba(0, 0, 0, 0.05);
    border-radius: 3px;
    height: 32px;
}

.title {
    /* display: inline-block; */
    flex: 1;
    align-items: center;
    padding: 0;
    margin: 0;
    text-align: center;
    font-size: 14px;
    line-height: 32px;
    transition: 150ms;
    color: #555;
}


.divider {
    background: red;
    width: 32px;
    position: relative;
}

.divider::after {
    content: "";
    border-bottom: 1px solid #888;
    position: absolute;
    left: 0;
    top: 0;
    width: 12px;
    transform: translateY(-4.24px) rotateZ(45deg);
}

.divider::before {
    content: "";
    border-bottom: 1px solid #888;
    position: absolute;
    left: 0;
    top: 0;
    width: 12px;
    transform: translateY(4.24px) rotateZ(-45deg);
}

.current {
    color: green;
}

.past {
    color: blue;
}

.future {
    color: gray;
}
</style>