export function estimate(iterCount) {
    return Math.floor(iterCount * 0.006);
}

export function toChs(iterCount) {
    let time = Math.floor(0.006 * iterCount / 1000);
    if (time < 1) {
        return "不到1秒";
    } else {
        let min = Math.floor(time / 60);
        if (min === 0) {
            return `${time}秒`;
        }
        return `${min}分${Math.floor(time - 60 * min)}秒`;
    }
}