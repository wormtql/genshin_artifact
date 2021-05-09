<template>
    <div class="migrate">
        <h1 style="margin: 0"><i class="el-icon-info"></i>重要通知</h1>
        <p>{{ `域名 "${host}" 已不再使用，将在近期停用，请尽快转移至新的域名` }}</p>
        <a
            :href="url"
            target="_blank"
            class="el-button el-button--danger"
        >
            <i class="el-icon-link"></i>
            {{ url }}
        </a>
    </div>
</template>

<script>
export default {
    name: "MigrateNotification",
    data() {
        return {
            url: "https://www.mona-uranai.com",
            host: location.hostname,
        }
    }
}
</script>

<style lang="scss" scoped>
.migrate {
    background-color: #fef0f0;
    color: #f56c6c;
    padding: 12px;
    border-radius: 3px;
}
</style>