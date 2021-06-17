export default function (tree, func, def = null) {
    let q = [tree];
    while (q.length > 0) {
        let p = q.pop();
        if (func(p)) {
            return p;
        }

        let children = p.children ?? [];
        for (let child of children) {
            q.push(child);
        }
    }

    return def;
}