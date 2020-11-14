/* See https://codepen.io/dakom/pen/WNxYrQM */
/* This is slightly adapted to work on a containing element instead of window */

const STAGE_WIDTH = 1920;
const STAGE_HEIGHT = 1080;
const STAGE_PADDING_Y_PERC = 0.05; // in percentage, to offset the stage area a bit
const STAGE_PADDING_X_PERC = 0.05;

let cancelListener = null;

export function cancelResizer() {
    if(cancelListener != null) {
        cancelListener();
    }
}

export function startResizerOnElement(element) {
    //dirty trick to wait till next paint
    requestAnimationFrame(() => {
        function resizeFit() {
            if(!element || !element.clientWidth || !element.clientHeight) {
                cancelResizer();
                return;
            }
            const targetRatio = STAGE_WIDTH / STAGE_HEIGHT;
            let width = element.clientWidth;
            let height = element.clientHeight;
            const windowRatio = width / height;

            if (windowRatio > targetRatio ) {
                width = height * targetRatio;
            } else {
                height = width / targetRatio;
            }

            const x = (element.clientWidth - width) / 2;
            const y = (element.clientHeight - height) / 2;
            const scale = width / STAGE_WIDTH;

            //document.documentElement.style.setProperty('font-size', `calc(62.5% * ${scale})`);

            //help fix safari, use hard pixel values
            document.documentElement.style.setProperty('font-size', `calc(10px * ${scale})`);
            document.documentElement.style.setProperty('--scale', `${scale}`);
            document.documentElement.style.setProperty('--x', `${x}px`);
            document.documentElement.style.setProperty('--y', `${y}px`);
            document.documentElement.style.setProperty('--width', `${width}px`);
            document.documentElement.style.setProperty('--height', `${height}px`);
            document.documentElement.style.setProperty('--content-x', `${(STAGE_PADDING_X_PERC/2) * width}px`);
            document.documentElement.style.setProperty('--content-y', `${(STAGE_PADDING_Y_PERC/2) * height}px`);
            document.documentElement.style.setProperty('--content-width', `${width - (STAGE_PADDING_X_PERC * width)}px`);
            document.documentElement.style.setProperty('--content-height', `${height - (STAGE_PADDING_Y_PERC * height)}px`);
        }

        cancelResizer();
        resizeFit();
        window.addEventListener("resize", resizeFit);
    });
}