// converts a string with newlines to an array where each line and each newline is an entry
// input "Hello\nto\nyou"
// output ["Hello", "\n", "to", "\n", "you"]
export function textToLineArray(str: string): string[] {
    const originalArr = str.split("\n");
    const newArr: string[] = [];

    for (let i = 0; i < originalArr.length - 1; i++) {
        newArr.push(originalArr[i]);
        newArr.push("\n");
    }
    newArr.push(originalArr[originalArr.length - 1]);

    return newArr;
}
