export const deleteNone = (args:any):any => {
    return (Object as any)
        .entries(args)
        .filter(([key, value]) => value !== "none")
        .reduce((acc, [key, value]) => Object.assign(acc, {[key]: value}), {})
}

export const argsToAttrs = (args:any):string => {
    return (Object as any)
    .entries(args)
        .filter(([key, value]) => value != null)
        .map(argToAttr)
        .filter(x => x != null)
        .join(' ')
}

export const argToAttr = ([key, value]:[string, any]):string => {
    const valueType = typeof value;
    switch(valueType) {
        case "boolean": return value ? key : null;
        case "string": 
        case "number":
            return `${key}="${value}"`;
        default: throw new Error(`can't turn ${valueType} into an attribute`);
    } 
    
}

