<template>
    <div>
        <div class="calculator-item">
            <h3 class="calculator-title">普通攻击-云来剑法</h3>
            
        </div>
        <el-table>

        </el-table>
        {{ configObject }}
    </div>
</template>

<script>
import Enemy from "@asset/enemies/enemy";

export default {
    name: "Keqing.calculator",
    props: {
        enemy: {
            type: Object,
            default: function () {
                return new Enemy("hilichurl", 80);
            }
        },
        configObject: {
            type: Object,
        }
    },
    data() {
        return {
            skillType: "a",
        }
    }
}
</script>