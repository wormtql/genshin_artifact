export class RandomIDProvider {
    generateId(): number {
        return Math.floor(Math.random() * Number.MAX_SAFE_INTEGER)
    }
}

export class AutoIncrementIDProvider {
    nextId: number = 1

    constructor() {
    }

    updateId(id: number) {
        if (id >= this.nextId) {
            this.nextId = id + 1
        }
    }

    generateId(): number {
        this.nextId += 1
        return this.nextId - 1
    }
}