/* See https://codepen.io/dakom/pen/WNxYrQM */

export interface ResizeStageConfig {
    width: number, 
    height: number, 
    paddingX: number, 
    paddingY: number, 
    marginX: number, 
    marginY: number
}
export type OnResize = (info:ResizeInfo) => any;

export interface ResizeInfo {
    scale: number,
    x: number,
    y: number,
    width: number,
    height: number,
    contentX: number,
    contentY: number,
    contentWidth: number,
    contentHeight: number
}

export const setResizeOnDocumentRoot = (info:ResizeInfo) => {
    setResizeOnStyle(document.documentElement.style, info);
}

export const setResizeOnStyle = (style: CSSStyleDeclaration, info:ResizeInfo) => {
    const {scale, x, y, width, height, contentX, contentY, contentWidth, contentHeight} = info;

            //would be nice if we could do this to accommodate browser settings
            //but it breaks in Safari iirc
            //style.setProperty('font-size', `calc(62.5% * ${scale})`);
   
    const fontSize = 1 * scale;
    style.setProperty('font-size', `${fontSize}px`);
    style.setProperty('--scale', `${scale}`);
    style.setProperty('--x', `${x}px`);
    style.setProperty('--y', `${y}px`);
    style.setProperty('--width', `${width}px`);
    style.setProperty('--height', `${height}px`);
    style.setProperty('--content-x', `${contentX}px`);
    style.setProperty('--content-y', `${contentY}px`);
    style.setProperty('--content-width', `${contentWidth}px`);
    style.setProperty('--content-height', `${contentHeight}px`);
}

const sizeEqual = (s1:ResizeInfo, s2:ResizeInfo):boolean => {
    return s1.scale === s2.scale
        && s1.x === s2.x
        && s1.y === s2.y
        && s1.width === s2.width
        && s1.height === s2.height
        && s1.contentX === s2.contentX
        && s1.contentY === s2.contentY
        && s1.contentWidth === s2.contentWidth
        && s1.contentHeight === s2.contentHeight
}

type CancelFn = () => any;
type ResizeObserver = any;
type ReturnTuple = [ResizeObserver, CancelFn];

export interface Options {
    stage: ResizeStageConfig,
    container?: Element | null,
    observeTargets?: Array<Element | null | undefined>,
    ignoreWindow?: boolean,
    adjustBounds?: (rect:DOMRect) => DOMRect,
}

export function startResizer({container, ignoreWindow, observeTargets, adjustBounds, stage}:Options, onResize: OnResize):ReturnTuple {
    let lastInfo:ResizeInfo = {
        scale: 0,
        x: 0,
        y: 0,
        width: 0,
        height: 0,
        contentX: 0,
        contentY: 0,
        contentWidth: 0,
        contentHeight: 0,
    }

    const resize = () => {
        const containerBounds = container 
            ? container.getBoundingClientRect() 
            : new DOMRect(0, 0, window.innerWidth, window.innerHeight);

        if (!containerBounds || !containerBounds.width || !containerBounds.height) {
            return;
        }

        let bounds = adjustBounds ? adjustBounds(containerBounds) : containerBounds;

        bounds = new DOMRect(
            bounds.x + stage.marginX, 
            bounds.y + stage.marginY,
            bounds.width - (stage.marginX * 2),
            bounds.height - (stage.marginY * 2)
        );
        const targetRatio = stage.width / stage.height;

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
        const scale = width / stage.width;

        const info = {
            scale,
            x,
            y,
            width,
            height,
            contentX: stage.paddingX,
            contentY: stage.paddingY,
            contentWidth: width - (stage.paddingX * 2),
            contentHeight: height - (stage.paddingY * 2)
        };

        console.log(info);

        if(!sizeEqual(info, lastInfo)) {
            onResize(info);
            lastInfo = info;
        }
    }

    // @ts-ignore
    const observer = new ResizeObserver(resize);

    if(observeTargets && observeTargets.length) {
        observeTargets.forEach(target => {
            if(target) {
                observer.observe(target);
            }
        })
    }

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
