export const arrayCount = (n:number) => 
    Array(n)
        .fill(null)
        .map((_, idx) => idx+1);