export default {
    amp: function (em) {
        return em * 25 / (9 * (em + 1400));
    },
    jubian: function (em) {
        return em * 20 / (3 * (em + 1400));
    }
}