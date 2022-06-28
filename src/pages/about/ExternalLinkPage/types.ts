export interface LinkItem {
    name: string,
    tags: string[],
    url: { title: string, link: string, icon: any, primary: boolean }[],
    description?: string,
}