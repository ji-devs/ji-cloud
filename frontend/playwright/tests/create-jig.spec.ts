import { test, expect, Page } from "@playwright/test";

test("create and delete jig", async ({ page }) => {
    await login(page);

    await createJig(page);

    await deleteAllJigs(page);
});

async function login(page: Page) {
    // open jigzi
    await page.goto("https://jigzi.org/");

    // navigate to login
    const login = page.locator(`page-header button-rect:text("Login") a`);
    await login.click();

    // check if we're on the login page
    // await expect(page).toHaveURL("/user/login");
    expect(page.url()).toContain("/user/login");

    // get the form elements
    const emailInput = page.locator(`page-login-landing input-wrapper[slot="email"] input`);
    const passwordInput = page.locator(`page-login-landing input-password[slot="password"] input-wrapper input`);
    const submitButton = page.locator(`page-login-landing button-rect-icon`);

    // fill the form
    await emailInput.fill("jitestlb@gmail.com");
    await passwordInput.fill("Ji2022LB");

    // submit the form
    await submitButton.click();

    // check if logged in successfully
    const shalomText = await page.locator("page-header-profile .main .name").textContent();
    await expect(shalomText).toContain("Shalom");
}


async function createJig(page: Page) {
    // navigate to gallery
    const createLink = page.locator(`page-header-link a[href*="gallery"]`);
    await createLink.click();

    // check if we're on the gallery page
    // expect(page).toHaveURL('/jig/edit/gallery');
    expect(page.url()).toContain("/jig/edit/gallery");

    // create a new jig
    const createJigLink = page.locator(`jig-gallery-create`);
    await createJigLink.click();

    await page.waitForNavigation();

    // check if we're on the jig edit page
    const uuid = "[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}";
    // await expect(page).toHaveURL(new RegExp(`jig\/edit\\${uuid}\b`));
    expect(page.url()).toMatch(new RegExp(`.*jig\/edit\/${uuid}.*`));

    // drag cover
    const coverModuleOption = page.locator(`jig-edit-module-card[module="cover"]`);
    await coverModuleOption.hover();
    const coverModuleSlot = page.locator(`jig-edit-sidebar-module-window[coveronly]`);
    await coverModuleSlot.waitFor();
    await coverModuleOption.dragTo(coverModuleSlot);

    // get module iframe
    page.locator(`iframe[src*="/module/"]`).waitFor();
    const moduleIframe = await page.frameLocator(`iframe[src*="/module/"]`);

    // change to content page
    const contentStep = moduleIframe.locator(`step-nav`).nth(2);
    await contentStep.evaluate(el => el.style.display = "block");
    await contentStep.click();

    // make sure there are not wysiwyg outputs yet
    const wysiwygOutput = moduleIframe.locator(`wysiwyg-output-renderer`);
    expect(await wysiwygOutput.count()).toBe(0);

    // insert text
    const insertTextButton = moduleIframe.locator(`text-editor-controls-insert-button`);
    await insertTextButton.click();

    // make sure that there are 2 wysiwyg-output-renderer, one for measuring text size and one for display
    expect(await wysiwygOutput.count()).toBe(2);

    // trigger focus
    const transformBox = moduleIframe.locator(`transform-box`);
    await transformBox.dispatchEvent("transform-rect-dblclick");

    // make sure that wysiwyg-base is on the page
    const wysiwygBase = moduleIframe.locator(`wysiwyg-base`);
    expect(await wysiwygBase.count()).toBe(1);

    // wait for the module to be saved
    await page.waitForResponse(/\/v1\/jig\/.{36}\/draft\/module/);

    // navigate ro publish
    const publishLink = page.locator(`jig-edit-sidebar-publish`);
    await publishLink.click();

    // make sure we're on the publish page
    expect(page.url()).toContain("/publish");

    // fill jig name
    const nameInput = page.locator(`input-wrapper[slot="name"] input`);
    await nameInput.fill("Test JIG");

    // publish
    const submitButton = page.locator(`jig-edit-publish [slot="publish"] button-rect`);
    await submitButton.click();

    // wait for the publish to finish
    await page.waitForResponse(/\/v1\/jig\/.{36}\/draft\/publish/);

    // make sure that the post-publish element is present on the page
    const postPublish = page.locator(`post-publish`);
    expect(await postPublish.count()).toBe(1);
}


async function deleteAllJigs(page: Page) {
    // navigate to the home page
    const homeLink = page.locator(`jig-edit-sidebar-header .logo-nav-wrapper a`);
    await homeLink.click();

    // navigate to gallery
    const createLink = page.locator(`page-header-link a[href*="gallery"]`);
    await createLink.click();

    // get all the jigs on the page
    const jigs = page.locator(`jig-gallery-recent`);
    await jigs.first().waitFor();
    const totalJigs = await jigs.count();

    // loop through all the jigs
    for (let i = 0; i < totalJigs; i++) {
        // get first jig
        const jig = jigs.first();

        // hover jig to show the menu
        await jig.hover();

        // get menu
        const menu = jig.locator(`menu-kebab`);

        // click the menu button
        const menuButton = menu.locator(`button-icon`);
        await menuButton.click();

        // click the delete option
        const deleteButton = jig.locator(`menu-line :text("Delete")`);
        await deleteButton.click();

        // click the confirm delete button
        const deleteConfirmButton = page.locator(`modal-confirm button-rect:text("Delete JIG")`);
        await deleteConfirmButton.click();

        // wait for the delete request for complete
        await page.waitForResponse(res => {
            return (
                /\/v1\/jig\/.{36}/.test(res.url())
                &&
                res.status() === 204
            )
        });
    }
}
