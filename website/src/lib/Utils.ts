type NestedObject = {
    [key: string]: unknown
}
export function isEmpty(obj: NestedObject) {
    for (const prop in obj) {
        if (Object.hasOwn(obj, prop)) {
            return false;
        }
    }
    return true;
}