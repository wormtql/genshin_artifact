export default {
    amp: function (em) {
        return em * 25 / (9 * (em + 1400));
    },
    transformative: function (em) {
        return em * 16 / (em + 2000);
    }
}