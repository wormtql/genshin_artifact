import { client } from "./client"

export async function getRepo(code) {
    return await client.get(`/repo/${code}`)
}

export async function createRepo(content) {
    return await client.post("/repo/create", content, {
        headers: {
            "Content-Type": "text/plain"
        }
    })
}