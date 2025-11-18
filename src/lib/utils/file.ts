export async function fileToBytes(file: File): Promise<Uint8Array | undefined> {
    if (!file) return undefined;

    // @ts-ignore
    return await file.bytes();
}
