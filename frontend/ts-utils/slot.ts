export const extractSlotStr = (slotName:string | undefined | null) => (obj:any):string => {
  return slotName && slotName != "" ? `slot="${slotName}"` : "";
}

interface HasOptionalSlot {
    slot?: string
}
export const injectSlotStr = <T extends HasOptionalSlot>(obj:T):T & {slotStr:string} => {
    const slotStr = extractSlotStr (obj.slot) (obj);
    return {...obj, slotStr};
}