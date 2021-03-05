export function random01() {
    return Math.random() < 0.5 ? 1 : 0;
}

export function randomElement(arr) {
    let index = Math.floor(Math.random() * arr.length);
    return arr[index];
}

export function randomIntBetween(l, r) {
    return Math.floor(Math.random() * (r - l)) + l;
}