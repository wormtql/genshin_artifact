<template>
    <div>
        <el-breadcrumb>
            <el-breadcrumb-item>更新记录</el-breadcrumb-item>
        </el-breadcrumb>
        <el-divider></el-divider>

        <el-timeline reverse>
            <el-timeline-item
                v-for="(change, index) in changelogs"
                :key="index"
                :timestamp="`${change.time} ---- V${change.version}`"
                placement="top"
            >
                <el-card>
                    <ul style="list-style-type: disc">
                        <li
                            v-for="(changeitem, index) in change.changes"
                            :key="index"
                        >{{ changeitem }}</li>
                    </ul>
                </el-card>
                <!-- <div class="item" :class="{ active: index === changelogs.length - 1 }">
                    
                </div> -->
            </el-timeline-item>
        </el-timeline>
    </div>
</template>

<script>
import changelogs from "@/changelog/changelog";

export default {
    name: "ChangeLogPage",
    created: function() {
        this.changelogs = changelogs;
    }
}
</script>

<style scoped>
.item p {
    padding: 0;
    margin: 0;
}

.item {
    background: rgba(0, 0, 0, 0.01);
    border: 1px solid #999;
    padding: 12px 0;
    margin-right: 64px;
    border-radius: 3px;
    color: #303133;
}

.item.active {
    background: rgba(225, 243, 216, 0.5);
    border: 1px solid #67c23a;
    
}

.item h2 {
    padding: 4px 20px 12px 20px;
    margin: 0;
    
    /* font-size: 14px; */
}

/* .item h2.active {
    font-size: 20px;
} */
</style>