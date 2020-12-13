/* See https://codepen.io/dakom/pen/WNxYrQM */
/* This is slightly adapted to work on a containing element instead of window */


//See: https://codepen.io/dakom/pen/WNxYrQM
import { STAGE_WIDTH, STAGE_HEIGHT, STAGE_PADDING_X_PERC, STAGE_PADDING_Y_PERC } from "../../config/js/src/lib";

/* Example Usage */
//const [observer, cancelObserver] = startResizer(document.getElementById("container"), false);
//observer.observe(document.getElementById("sidebar"));


//THE FUNCTION
export function startResizer(container, ignoreWindow) {
    const resize = () => {
        const bounds = container ? container.getBoundingClientRect() : null;
        if (!bounds || !bounds.width || !bounds.height) {
            return;
        }

        const targetRatio = STAGE_WIDTH / STAGE_HEIGHT;

        let width = bounds.width;
        let height = bounds.height;

        const ratio = width / height;

        if (ratio > targetRatio) {
            width = height * targetRatio;
        } else {
            height = width / targetRatio;
        }

        const x = bounds.x + ((bounds.width - width) / 2);
        const y = bounds.y + ((bounds.height - height) / 2);
        const scale = width / STAGE_WIDTH;

        //would be nice if we could do this to accommodate browser settings
        //but it breaks in Safari iirc
        //document.documentElement.style.setProperty('font-size', `calc(62.5% * ${scale})`);
        document.documentElement.style.setProperty('font-size', `calc(10px * ${scale})`);
        document.documentElement.style.setProperty('--scale', `${scale}`);
        document.documentElement.style.setProperty('--x', `${x}px`);
        document.documentElement.style.setProperty('--y', `${y}px`);
        document.documentElement.style.setProperty('--width', `${width}px`);
        document.documentElement.style.setProperty('--height', `${height}px`);
        document.documentElement.style.setProperty('--content-x', `${(STAGE_PADDING_X_PERC / 2) * width}px`);
        document.documentElement.style.setProperty('--content-y', `${(STAGE_PADDING_Y_PERC / 2) * height}px`);
        document.documentElement.style.setProperty('--content-width', `${width - (STAGE_PADDING_X_PERC * width)}px`);
        document.documentElement.style.setProperty('--content-height', `${height - (STAGE_PADDING_Y_PERC * height)}px`);

        //note we could also stash the current vars in JS for non-CSS usage like canvas, absolute positioning of other layers, etc.
    }

    // @ts-ignore
    const observer = new ResizeObserver(resize);
    if (!ignoreWindow) {
        window.addEventListener("resize", resize);
    }

    const cancel = () => {
        observer.disconnect();
        if (!ignoreWindow) {
            window.removeEventListener("resize", resize);
        }
    }

    resize();
    return [observer, cancel];
}