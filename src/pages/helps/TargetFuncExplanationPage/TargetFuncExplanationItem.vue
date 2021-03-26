<template>
    <div class="fs-12">
        <span class="name">{{ name }}</span>
        <div style="margin: 8px 0">
            <span class="des">{{ description }}</span>
        </div>
    </div>
</template>

<script>
export default {
    name: "TargetFuncExplanationItem",
    props: ["name", "description"],
}
</script>

<style scoped>
.name {
    color: white;
    background: #123456aa;
    padding: 4px;
    border-radius: 3px;
}
</style>