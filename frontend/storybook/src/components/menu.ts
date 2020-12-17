import "@elements/menu/menu-tab";
import {Menutab} from "~/components/menu/menu-tab";
export default {
  title: 'Menu Tab',
}

export const Menu = () => {
    return `<menu-wrapper>
    <menu-tab label="Menu" slot="one"></menu-tab>
    <menu-tab label="Home" slot="two"></menu-tab>
    </menu-wrapper>
 
    `
}