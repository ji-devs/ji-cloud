import "@elements/core/titles/variants/underlined-title";
import "@elements/core/dividers/or-divider";
import "@elements/core/cards/grey";
import "@elements/core/lists/list-vertical";
import {AGE_OPTIONS} from "~/mock/meta";

export default {
  title: 'Entry / Admin / Images / Meta / Lists'
  
}


export const UserInfo = () => {



    return `
    <grey-card>
       <div slot="content">${AGE_OPTIONS}</div>
    </grey-card>
    
    `
}


