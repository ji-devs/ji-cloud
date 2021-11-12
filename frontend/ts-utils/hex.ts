export function hexStringToNumber(s: string): number {
    //strip off leading 0x since the 0 would otherwise match
    s = s.indexOf("0x") === 0 ? s.substr(2) : s;

    let acc = "";
    //could be made simpler with regex but w/e
    for (let i = 0; i < s.length; i++) {
        const c = s.charAt(i).toLowerCase();
        switch (c) {
            case "0":
            case "1":
            case "2":
            case "3":
            case "4":
            case "5":
            case "6":
            case "7":
            case "8":
            case "9":
            case "a":
            case "b":
            case "c":
            case "d":
            case "e":
            case "f":
                acc += c;
                break;
            default:
                break;
        }
    }

    return parseInt(acc, 16);
}

export function hexNumberToRgb(n: number): [number, number, number] {
    return [(n >> 16) & 0xff, (n >> 8) & 0xff, n & 0xff];
}

export function hexRgbToNumber(r: number, g: number, b: number): number {
    return (r << 16) | (g << 8) | b;
}

export function hexNumberToString(n: number, prefix: string = "#"): string {
    return prefix + n.toString(16);
}
