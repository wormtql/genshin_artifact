(function inBrowserGetCharacterStats() {
    const tables = [...document.querySelectorAll("table.genshin_table.stat_table")]
    const hpValues = []
    const atkValues = []
    const defValues = []
    if (tables && tables.length > 0) {
        const table = tables[0];
        let rows = [...table.querySelectorAll("tr")]
        rows.shift()
        for (const row of rows) {
            const cells = row.querySelectorAll("td")
            const hpCell = cells[1]

            const hp = parseFloat(hpCell.textContent);
            hpValues.push(hp);

            const atkCell = cells[2]
            const atk = parseFloat(atkCell.textContent);
            atkValues.push(Math.round(atk))

            const defCell = cells[3]
            const def = parseFloat(defCell.textContent);
            defValues.push(Math.round(def))
        }
    }

    console.log(hpValues)
    console.log(atkValues)
    console.log(defValues)
})()
