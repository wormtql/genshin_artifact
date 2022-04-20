import axios from "axios"
import { client } from "./client"


export async function createFeedback(feedback) {
    return await client.post("/feedbacks/create", {
        text: feedback
    })
}

export async function createComputeResult(characterInterface, weaponInterface, buffsInterface, targetFunctionInterface, resultArtifacts) {
    const config = {
        character: characterInterface,
        weapon: weaponInterface,
        buffs: buffsInterface,
        target_function: targetFunctionInterface
    }

    const data = {
        config,
        result_artifacts: resultArtifacts
    }

    return await client.post("/compute_result/create", data)
}

export async function getComputeResultAnalysis() {
    return await client.get("/compute_result/analysis")
}
