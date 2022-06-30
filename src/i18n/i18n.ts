import inlineLocale from "./locales/zh-cn"
import {pathAccess} from "@/utils/common"


export function createI18n() {
    const locale = ref("zh-cn")
    const fallbackLocale = ref("zh-cn")
    const messages: Record<string, any> = {
        "zh-cn": inlineLocale
    }

    function t(...s: (string | number)[]) {
        const msg = messages[locale.value]
        const value = pathAccess(msg, ...s)
        if (!value) {
            const msg2 = messages[fallbackLocale.value]
            const value2 = pathAccess(msg2, ...s)
            if (!value2) {
                return s[0]
            } else {
                return value2
            }
        } else {
            return value
        }
    }

    async function setLocale(name: string) {
        await setLocaleMessage(name)
        locale.value = name
    }

    async function setLocaleMessage(name: string) {
        if (!messages[name]) {
            const message = await import(
                /* webpackChunkName: "locale-[request]" */
                `./locales/${name}`
                )
            messages[name] = message.default
        }
    }

    return {
        locale,

        t,
        setLocale,
    }
}

const i18n = createI18n()

export function useI18n() {
    return i18n
}

// export function createI18n() {
//     const options = {
//         locale: "zh-cn",
//         fallbackLocale: "zh-cn",
//         legacy: false,
//         messages: {
//             "zh-cn": inlineLocale
//         }
//     }
//
//     const i18n = createI18n1(options)
//     return i18n
// }

// const { locale, setLocaleMessage } = useI18n()


// export function setLocale(i18n: ReturnType<typeof createI18n1>, name: string): void {
//     if (i18n.mode === "legacy") {
//         i18n.global.locale = name
//     } else {
//         // @ts-ignore
//         i18n.global.locale.value = name
//     }
// }

// export async function setLocalMessage(name: string) {
//     const message = await import(
//         /* webpackChunkName: "local-[request]" */
//         `./locales/${name}`
//         )
//     setLocaleMessage(name, message)
//
//     locale.value = name
// }