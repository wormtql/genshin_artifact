import axios from "axios"


const _client = axios.create({
    baseURL: "api/",
    timeout: 1000
})

export const client = _client
