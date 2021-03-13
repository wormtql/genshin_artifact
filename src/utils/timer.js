export default function (prom) {
    let start = new Date();

    return new Promise((resolve, reject) => {
        prom.then(() => {
            let end = new Date();
            resolve(end - start);
        }).catch(() => {
            reject();
        });
    });
}