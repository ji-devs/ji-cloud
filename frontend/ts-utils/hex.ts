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

export function rgbToHsl(r: number, g: number, b: number): [number, number, number] {
    // Make r, g, and b fractions of 1
    r /= 255;
    g /= 255;
    b /= 255;

    // Find greatest and smallest channel values
    let cmin = Math.min(r,g,b);
    let cmax = Math.max(r,g,b);
    let delta = cmax - cmin;
    let h = 0;
    let s = 0;
    let l = 0;

    // Calculate hue
    // No difference
    if (delta === 0)
        h = 0;
    else if (cmax == r) // Red is max
        h = ((g - b) / delta) % 6;
    else if (cmax == g) // Green is max
        h = (b - r) / delta + 2;
    else // Blue is max
        h = (r - g) / delta + 4;

    h = +(h * 60).toFixed(1);

    // Make negative hues positive behind 360°
    if (h < 0)
        h += 360;

    // Calculate lightness
    l = (cmax + cmin) / 2;

    // Calculate saturation
    s = delta == 0 ? 0 : delta / (1 - Math.abs(2 * l - 1));


    // Multiply l and s by 100
    s = +(s * 100).toFixed(1);
    l = +(l * 100).toFixed(1);

    return [h, s, l];
}
