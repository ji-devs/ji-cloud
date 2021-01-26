import "@elements/entry/home/TOSORT/create-leftparagraph";
import "@elements/entry/home/sections/footer-section";
import "@elements/core/dividers/stripe-along";
import "@elements/entry/home/social-networks";
import "@elements/entry/home/TOSORT/footer-kidsafe"; 

  import {footerjigs} from "~/components/entry/homepage/footer/footer-jigs";
  import {footerhelp} from "~/components/entry/homepage/footer/footer-help";
  import {footerproduct} from "~/components/entry/homepage/footer/footer-product";
  import {footercontactus} from "~/components/entry/homepage/footer/footer-contactus";
  import {footerwhoweare} from "~/components/entry/homepage/footer/footer-whoweare";
  import {footerproductsservices} from "~/components/entry/homepage/footer/footer-productsservices";

export default {
  title: 'Homepage',
}


const STR_lightBlue = "lightBlue";
const STR_TITLEJi="Jewish Interactive";
const STR_TITLEAbout="About Us";
const STR_SIZEMEDIUM="medium";
const STR_PATHKIDSAFE="logo-kid-safe.jpg";
const STR_TERMS="Terms & Conditions";
const STR_PRIVACY="Privacy Policy";

export const footer = () => {
    return `
    <footer-section >   
    <title-section titlecolor="${STR_lightBlue}" title="${STR_TITLEJi}" size="${STR_SIZEMEDIUM}" slot="titleJi"></title-section>
    <title-section titlecolor="${STR_lightBlue}" title="${STR_TITLEAbout}" size="${STR_SIZEMEDIUM}" slot="titleAbout"></title-section>
    <div slot="footer-column">
        ${footerjigs()}
    </div>
    <div slot="footer-column">
${footerproduct()}
</div>
    <div slot="footer-column">
    ${footerhelp()}
</div>
<div slot="footer-column">
${footercontactus()}
</div>
<stripe-along slot="footer-column"></stripe-along>
<div slot="footer-column">
${footerwhoweare()}
</div>
   <div slot="footer-column">
        ${footerproductsservices()}
    </div>
<footer-kidsafe slot="kidsafe" path_kidsafe="${STR_PATHKIDSAFE}" term="${STR_TERMS}" privacy="${STR_PRIVACY}" ></footer-kidsafe>
   </footer-section>
    `
}
