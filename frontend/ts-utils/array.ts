// gives an array of numbers starting at 1 (not 0) until the provided max
export const arrayCount = (max: number) =>
    Array(max)
        .fill(null)
        .map((_, idx) => idx + 1);

// gives an array of numbers starting at 0 until the provided max
export const arrayIndex = (max: number) =>
    Array(max)
        .fill(null)
        .map((_, idx) => idx);

//maps an array of things into a flat string
//requires that the mapping function return a string for each thing
//useful for showing a list of elements in storybook
//because an actual array would just be rendered as a weird object
export const mapToString = <T>(
    arr: Array<T>,
    fn: (value: T, index?: number) => string
): string => {
    return arr.map(fn).reduce((acc, curr) => acc + curr, "");
};
