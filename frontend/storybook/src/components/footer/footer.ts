import "@elements/icon-wtitle-wparagraph";
import "@elements/homepage-sections/footer-section";
import "@elements/column-details";
import "@elements/column-list";

  import {footerjigs} from "~/components/footer/footer-jigs";
  import {footerhelp} from "~/components/footer/footer-help";
  import {footerproduct} from "~/components/footer/footer-product";
  import {footercontactus} from "~/components/footer/footer-contactus";
  import {footerwhoweare} from "~/components/footer/footer-whoweare";
  import {footerproductsservices} from "~/components/footer/footer-productsservices";


export default {
  title: 'Homepage Paragraph',
}

const STR_lightBlue = "lightBlue";
const STR_TITLEJi="Jewish Interactive";
const STR_TITLEAbout="About Us";


export const footer = () => {
    return `

    <footer-section >   
    <title-section titlecolor="${STR_lightBlue}" title="${STR_TITLEJi}" slot="title"></title-section>

    <title-section titlecolor="${STR_lightBlue}" title="${STR_TITLEAbout}" slot="title"></title-section>

    
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
<div slot="footer-column">
${footerwhoweare()}
</div>
   <div slot="footer-column">
        ${footerproductsservices()}
    </div>

</footer-section>
    `
}