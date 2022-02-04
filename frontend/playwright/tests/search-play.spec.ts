import { test, expect } from "@playwright/test";

test("basic search", async ({ page }) => {
    await page.goto("https://jigzi.org/");

    // trigger a search
    const searchButton = await page.locator("home-search-bar button-rect");
    await searchButton.click();

    // make sure that there are 20 jig results
    const jigResults = await page.locator("home-search-results-section[kind=jigs] home-search-result").count();
    await expect(jigResults).toBe(20);
});

test("search and play a jig", async ({ page }) => {
    await page.goto("https://jigzi.org/");

    // trigger a search
    const searchButton = await page.locator("home-search-bar button-rect");
    await searchButton.click();

    // play first result
    const jigResult = await page.locator("home-search-results-section[kind=jigs] home-search-result").first();
    // jigResult.waitFor();
    await jigResult.hover();
    const jigResultButton = await jigResult.locator("button-rect[slot=play-button]");
    await jigResultButton.click();

    // wait for player iframe
    page.waitForSelector('iframe[src*="/jig/play"]');
    const jigIframe = await page.frameLocator(`iframe[src*="/jig/play"]`);

    // get module iframe
    const moduleIframe = await jigIframe.frameLocator(`iframe[src*="/module/"]`);

    // make sure that the module-page-iframe element is on the page
    const modulePageElement = await moduleIframe.locator("module-page-iframe");
    await modulePageElement.waitFor();
    expect(await modulePageElement.count()).toBe(1);
});
