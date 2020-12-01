//these tend to return the element they are operating on
//in order to allow a fluent/builder sort of pattern
//Everything is just typed to element for now... can get more specific later
export function appendId(parentElement:Element, id:string, child:Element) {
    const container = getChildId(parentElement, id);
    container.append(child);
    return parentElement;
}

export function prependId(parentElement:Element, id:string, child:Element) {
    const container = getChildId(parentElement, id);
    container.prepend(child);
    return parentElement;
}

export function setSrc(element:Element, src:string) {
    element.setAttribute("src", src);
    return element;
}
export function setSrcId(parentElement:Element, id:string, src:string) {
    setAttributeId(parentElement, id, "src", src);
    return parentElement;
}

export function addClassesId(parentElement:Element, id:string, classNames:string[] | string) {
    const container = getChildId(parentElement, id);
    const c = Array.isArray(classNames) ? classNames : [classNames];
    addClasses(container, c);
    return parentElement;
}

export function addClasses(element:Element, classNames:string[] | string) {
    const classList = element.classList;

    const c = Array.isArray(classNames) ? classNames : [classNames];
    classList.add(...c);
    return element;
}

export function removeClassesId(parentElement:Element, id:string, classNames:string[] | string) {
    const container = getChildId(parentElement, id);
    const c = Array.isArray(classNames) ? classNames : [classNames];
    removeClasses(container, c);
    return parentElement;
}

export function removeClasses(element:Element, classNames:string[] | string) {
    const classList = element.classList;

    const c = Array.isArray(classNames) ? classNames : [classNames];
    classList.remove(...c);
    return element;
}
export function setTextId(element:Element, id:string, text:string) {
    const container = getChildId(element, id);
    (container as any).innerText = text;
    return element;
}

export function appendTextId(element:Element, id:string, text:string) {
    const container = getChildId(element, id);
    container.textContent += text;
    return element;
}
export function appendTextLineId(element:Element, id:string, text:string) {
    const container = getChildId(element, id);
    container.textContent += text + '\n';
    return element;
}
export function setValueId(element:Element, id:string, text:string) {
    const container = getChildId(element, id);
    (container as any).value = text;
    return element;
}

export function appendValueLineId(element:Element, id:string, text:string) {
    const container = getChildId(element, id);
    (container as any).value += text + '\n';
    return element;
}
export function appendValueId(element:Element, id:string, text:string) {
    const container = getChildId(element, id);
    (container as any).value += text;
    return element;
}

export function setAttributeId(element:Element, id:string, attr:string, value:string) {
    const container = getChildId(element, id);
    container.setAttribute(attr, value);
    return element;
}

export function setIframeContentsId(element:Element, id:string, html:string) {
    return setIframeContents(getChildId(element, id), html);
}
export function setIframeContents(element:Element, html:string) {
    (element as any).srcdoc = html; 
    return element;
}

export function getChildId(element:Element, id:string, supressError?:boolean) {
    let child = element.querySelector(dataId(id));
    if(!supressError && !child) {
        console.error(`could not get child id for ${id}`);
    }

    return child;
}

export function dataId(id) {
    return `[data-id='${id}']`;
}
