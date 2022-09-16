import {computed, reactive, ref, watch} from "vue"

function f() {
    const sequence = ref<string[]>([])

    function init(payload: any) {
        if (payload) {
            sequence.value = payload.sequence
        } else {
            sequence.value = []
        }
    }

    return {
        sequence,
        init,
    }
}

const s = f()

export function watchContent() {
    return {
        sequence: s.sequence.value
    }
}

// watch(() => {
//     return s.sequence.value
// }, newValue => {
//     localStorage.setItem("sequence", JSON.stringify(newValue))
// }, {
//     deep: true
// })

export const useSequenceStore = () => {
    return s
}
