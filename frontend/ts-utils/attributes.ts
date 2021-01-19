export const argsToAttrs = (args:Object):string => {
    return (Object as any)
        .entries(args)
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