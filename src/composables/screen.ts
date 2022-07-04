function getWidth(): number {
    const w = Math.max(document.documentElement.clientWidth || 0, window.innerWidth || 0)
    return w
}

function screenWidth() {
    const width = ref(getWidth())

    function handleResize() {
        // width.value = e.
        width.value = getWidth()
    }

    const geSmall = computed(() => width.value >= 768)
    const geMedium = computed(() => width.value >= 992)

    // const isMobile = computed(() => )

    window.addEventListener("resize", handleResize)

    return {
        // isSmall,
        geSmall,
        geMedium,
        width,
    }
    // const width = ref(window.wid)
}

const s = screenWidth()

export function useScreen(): ReturnType<typeof screenWidth> {
    return s
}
