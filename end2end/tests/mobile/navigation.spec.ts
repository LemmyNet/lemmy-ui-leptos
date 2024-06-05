import { expect, test } from "@playwright/test";

test.beforeEach(async ({ page }) => {
  page.goto("/");
});

test("Successfully navigates around the page", async ({ page, baseURL }) => {
  const assertUrl = (path: string) =>
    expect(page.url()).toBe(`${baseURL}/${path}`);
  const mobileNav = page.getByLabel("Mobile nav");

  await mobileNav.getByRole("link", { name: "Communities" }).click();
  assertUrl("communities");

  await mobileNav.getByRole("link", { name: "Search" }).click();
  assertUrl("search");

  await mobileNav.getByRole("link", { name: "Saved" }).click();
  await page.waitForURL(`${baseURL}/saved`);
  assertUrl("saved");

  await mobileNav.getByRole("link", { name: "Home" }).click();
  await page.waitForURL(`${baseURL}/`);
  assertUrl("");
});
