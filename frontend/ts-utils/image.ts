export function sameOrigin(url: string): boolean {
    const is_web =
        url.indexOf("http://") === 0 || url.indexOf("https://") === 0;

    if (is_web) {
        const locationOrigin = window.location.origin;
        const urlOrigin = new URL(url).origin;
        return urlOrigin == locationOrigin;
    } else {
        return true;
    }
}
