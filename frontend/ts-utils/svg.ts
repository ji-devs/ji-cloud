const _suffix = (s: string) => (path: string) =>
    (path = "" ? s : `${path} ${s}`);

/*M = moveto
L = lineto
H = horizontal lineto
V = vertical lineto
C = curveto
S = smooth curveto
Q = quadratic Bézier curve
T = smooth quadratic Bézier curveto
A = elliptical Arc
Z = closepath
 */

export const pointsToPath = (points: Array<[number, number]>): string => {
    if (points.length > 0) {
        let output = `M${points[0][0]} ${points[0][1]}`;
        for (let index = 1; index < points.length; index++) {
            const x = points[index][0];
            const y = points[index][1];
            output += ` L${x} ${y}`;
        }
        return output + ` Z`;
    } else {
        return "";
    }
};
