type ArtifactItem = number | null

export interface KumiItem {
    id: number,
    title: string,
    dir: boolean,
    children?: number[],
    artifactIds?: [ArtifactItem, ArtifactItem, ArtifactItem, ArtifactItem, ArtifactItem]
}