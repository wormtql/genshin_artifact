import positions from "@const/positions";

export function count(artifacts) {
    let c = 0;

    positions.forEach(pos => {
        c += artifacts[pos].length;
    });

    return c;
}