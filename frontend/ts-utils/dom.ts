export function withSlot(slot: string, html: string): string {
    const getInsertPos = (): any => {
        for (let i = 1; i < html.length; i++) {
            if (!isNaN(html[i] as any)) {
                return i;
            }
        }
    };
    const splitPos = getInsertPos();
    const part_1 = html.substr(0, splitPos);
    const part_2 = html.substr(splitPos);
    return `${part_1} slot="${slot}" ${part_2}`;
}

/// From top down
export function queryPierceShadowChildren(
    nodes: NodeList,
    selector: string
): HTMLElement | null {
    //NodeList isn't Array, and Array.find() returns undefined instead of null
    //so regular ol' for loop it is...
    for (let i = 0; i < nodes.length; i++) {
        const result = queryPierceShadow(nodes[i], selector);
        if (result != null) {
            return result;
        }
    }
    return null;
}
export function queryPierceShadow(
    node: Node | null,
    selector: string
): HTMLElement | null {
    if (!node) {
        return null;
    }
    if (node instanceof ShadowRoot) {
        return queryPierceShadow(node.host, selector);
    }

    if (node instanceof HTMLElement) {
        let result = node.shadowRoot?.querySelector(selector);
        if (result instanceof HTMLElement) {
            return result;
        }
        result = node.querySelector(selector);
        if (result instanceof HTMLElement) {
            return result;
        }

        return queryPierceShadowChildren(node.childNodes, selector);
    }
    return queryPierceShadowChildren(node.childNodes, selector);
}

// difference between closestPierceShadow and closestPierceSlot:
// closestPierceShadow starts in a shadow and crawls out, while closestPierceSlot starts outside of the shadow on crawls in
// TODO - clarify that note... closestPierceShot is still crawling up through parents...
export function closestPierceShadow(
    node: Node | null,
    selector: string
): HTMLElement | null {
    if (!node) {
        return null;
    }
    if (node instanceof ShadowRoot) {
        return closestPierceShadow(node.host, selector);
    }
    if (node instanceof HTMLElement) {
        if (node.matches(selector)) {
            return node;
        } else {
            return closestPierceShadow(node.parentNode, selector);
        }
    }
    return closestPierceShadow(node.parentNode, selector);
}

export function closestPierceSlot(
    node: Node | null,
    selector: string
): Node | null {
    if (!node) {
        return null;
    } else {
        if (node instanceof ShadowRoot) {
            return closestPierceSlot(node.host, selector);
        }
        if (node instanceof HTMLElement) {
            if (node.matches(selector)) {
                return node;
            } else {
                return closestPierceSlot(
                    node.assignedSlot || node.parentNode,
                    selector
                );
            }
        }
        return node.parentNode; // not sure if this is reachable
    }
}

// https://stackoverflow.com/a/56105394/784519
/*
export function closestElement(selector: string, base: Element | Window | Document): Element | null {
  function __closestFrom(el: Element | Window | Document): Element | null{
    if (!el || el === document || el === window) return null;
    if ((el as any).assignedSlot) el = (el as any).assignedSlot;
    let found = (el as Element).closest(selector);
    return found
      ? found
      : __closestFrom(((el as Element).getRootNode() as ShadowRoot).host);
  }
  return __closestFrom(base);
}
*/

/* not using any of these any more
export function withSlot(slot:string, html:string):string {
    const getInsertPos = ():number => {
        for(let i = 1; i < html.length; i++) {
            if(!isNaN(html[i] as any)) {
                return i;
            }
        }
    }
    const splitPos = getInsertPos(); 
    const part_1 = html.substr(0, splitPos);
    const part_2 = html.substr(splitPos);
    return `${part_1} slot="${slot}" ${part_2}`;
}

export function makeElement(html:string):any {
    const template = document.createElement("template");
    template.innerHTML = html;
    return (template.content.cloneNode(true) as any).firstElementChild;
}


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

*/
