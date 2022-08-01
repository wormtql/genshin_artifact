import {computed, reactive, ref, watch} from "vue"

function f() {
    const sequence = ref<string[]>([])

    return {
        sequence
    }
}

const s = f()

watch(() => {
    return s.sequence
}, newValue => {
    localStorage.setItem("sequence", JSON.stringify(newValue))
}, {
    deep: true
})

export const useSequenceStore = () => {
    return s
}
