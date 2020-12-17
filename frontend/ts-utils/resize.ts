/* See https://codepen.io/dakom/pen/WNxYrQM */

import { STAGE_WIDTH, STAGE_HEIGHT, STAGE_PADDING_X_PERC, STAGE_PADDING_Y_PERC } from "@project-config";

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
   
    const fontSize = 10 * scale;
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

type CancelFn = () => any;
type ResizeObserver = any;
type ReturnTuple = [ResizeObserver, CancelFn];

export interface Options {
    container?: Element | null,
    observeTargets?: Array<Element | null | undefined>,
    ignoreWindow?: boolean,
    adjustBounds?: (rect:DOMRect) => DOMRect
}

export function startResizer({container, ignoreWindow, observeTargets, adjustBounds}:Options, onResize: OnResize):ReturnTuple {
    const resize = () => {
        const containerBounds = container 
            ? container.getBoundingClientRect() 
            : new DOMRect(0, 0, window.innerWidth, window.innerHeight);

        if (!containerBounds || !containerBounds.width || !containerBounds.height) {
            return;
        }

        const bounds = adjustBounds ? adjustBounds(containerBounds) : containerBounds;

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

        onResize({
            scale,
            x,
            y,
            width,
            height,
            contentX: (STAGE_PADDING_X_PERC / 2) * width,
            contentY: (STAGE_PADDING_Y_PERC / 2) * height,
            contentWidth: width - (STAGE_PADDING_X_PERC * width),
            contentHeight: height - (STAGE_PADDING_Y_PERC * height)
        })
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