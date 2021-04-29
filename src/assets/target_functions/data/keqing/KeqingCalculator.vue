<template>
    <div>
        
    </div>
</template>

<script>
import Enemy from "@enemy/enemy";

export default {
    name: "KeqingCalculator",
    props: {
        enemy: {
            type: Object,
            default: function () {
                return new Enemy("hilichurl", 80);
            }
        }
    },
    data() {
        return {
            skillType: "a",
        }
    }
}
</script>