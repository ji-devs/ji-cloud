import "@elements/entry/home/TOSORT/create-leftparagraph";
import "@elements/entry/home/sections/footer-section";
import "@elements/core/dividers/stripe-along";
import "@elements/entry/home/social-networks";
import "@elements/entry/home/TOSORT/footer-kidsafe"; 
import "@elements/entry/home/sections/contactus-section";
import "@elements/entry/home/sections/products-section";
import "@elements/entry/home/sections/whoweare-section";

  import {footerjigs} from "~/components/entry/homepage/footer/footer-jigs";
  import {footerhelp} from "~/components/entry/homepage/footer/footer-help";
  import {footerproduct} from "~/components/entry/homepage/footer/footer-product";
  import {footercontactus} from "~/components/entry/homepage/footer/footer-contactus";
  import {footerwhoweare} from "~/components/entry/homepage/footer/footer-whoweare";
  import {footerproductsservices} from "~/components/entry/homepage/footer/footer-productsservices";

export default {
  title: 'Entry/ Homepage / Section',
}

const STR_JEWISHINTERACTIVE="Jewish Interactive";
const STR_ABOUTUS="About Us";



export const footer = () => {
    return `
    <footer-section >   
    <title-section titlecolor="lightBlue" title="${STR_JEWISHINTERACTIVE}" size="medium" slot="titleJi"></title-section>
    <title-section titlecolor="lightBlue" title="${STR_ABOUTUS}" size="medium" slot="titleAbout"></title-section>
    <div slot="footer-column">
        ${footerjigs()}
    </div>
    <div slot="footer-column">
<products-section></products-section>
    </div>
    <div slot="footer-column">
    ${footerhelp()}
</div>
<div slot="footer-column">
 
<contactus-section ></contactus-section>
</div>
<stripe-along slot="footer-column"></stripe-along>
<div slot="footer-column">
<whoweare-section></whoweare-section>
</div>
   <div slot="footer-column">
        ${footerproductsservices()}
    </div>
<footer-kidsafe slot="kidsafe" ></footer-kidsafe>
   </footer-section>
    `
}
