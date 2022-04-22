import { UAParser } from "ua-parser-js"


const parser = new UAParser()
console.log(parser.getResult())


function _deviceIsPC() {
    const type = parser.getDevice().type
    return !type
}

export const deviceIsPC = _deviceIsPC()
console.log(deviceIsPC)
