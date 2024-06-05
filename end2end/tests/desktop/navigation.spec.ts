import { expect, test } from "@playwright/test";

test("Successfully navigates around the page", async ({ page }) => {
  await page.goto("/");

  await page.getByLabel("Create Post").click();
  expect(page.url()).toBe("/create_post");

  await page.getByLabel("Create Community").click();
  expect(page.url()).toBe("/create_community");

  await page.getByLabel("Communities").click();
  expect(page.url()).toBe("/communities");

  await page.getByLabel("Search").click();
  expect(page.url()).toBe("/search");

  await page.getByLabel("Modlog").click();
  expect(page.url()).toBe("/modlog");

  await page.getByLabel("Instances").click();
  expect(page.url()).toBe("/instances");

  await page.getByLabel("Legal").click();
  expect(page.url()).toBe("/legal");

  await page.getByText("lemmy-dev").click();
  expect(page.url()).toBe("/");
});
