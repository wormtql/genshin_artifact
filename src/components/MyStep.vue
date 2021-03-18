<template>
    <div class="my-step">
        <template
            v-for="(step, index) in steps"
        >
            <div
                :key="'title' + step"
                class="title"
                :class="getClass(index)"
                @click="handleClick(index)"
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
        },
        lock: {
            type: Boolean,
            default: true,
        }
    },
    methods: {
        handleClick(index) {
            if (!this.lock && index !== this.pointer) {
                this.$emit("navigate", index);
            } else if (index < this.pointer) {
                this.$emit("navigate", index);
            }
        },

        getClass(index) {
            if (this.lock) {
                return {
                    current: index === this.pointer,
                    past: index < this.pointer,
                    future: index > this.pointer,
                };
            } else {
                return {
                    current: index === this.pointer,
                    past: index !== this.pointer,
                };
            }
        }
    }
}
</script>

<style scoped>
.my-step {
    display: flex;
    background: #f5f7fa;
    border-radius: 3px;
    height: 48px;
}

.title {
    /* display: inline-block; */
    flex: 1;
    align-items: center;
    padding: 0;
    margin: 0;
    text-align: center;
    font-size: 14px;
    line-height: 48px;
    transition: 150ms;
    color: #555;
    margin: 0 16px;
}


.divider {
    width: 12px;
    position: relative;
}

.divider::after {
    content: "";
    border-bottom: 1px solid #888;
    position: absolute;
    left: 0;
    top: 50%;
    width: 12px;
    transform: translateY(-4.24px) rotateZ(45deg);
}

.divider::before {
    content: "";
    border-bottom: 1px solid #888;
    position: absolute;
    left: 0;
    top: 50%;
    width: 12px;
    transform: translateY(4.24px) rotateZ(-45deg);
}

.current {
    box-shadow: 0 0 20px 3px rgba(0, 0, 0, 0.1);
    /* transform: scale(1.1); */
    /* border: 1px solid rgb(46, 221, 221); */
    background: #f5f7fa;
    color: #303133;
    border-radius: 3px;
}

.past {
    color: #409eff;
    cursor: pointer;
}

.past:hover {
    color: #6eb7ff;
}

.future {
    color: #c0c4cc;
}
</style>