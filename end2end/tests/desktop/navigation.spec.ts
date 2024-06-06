import { expect, test } from "@playwright/test";

test("Successfully navigates around the page", async ({ page, baseURL }) => {
  await page.goto("/");

  const assertUrl = (path: string) =>
    expect(page.url()).toBe(`${baseURL}/${path}`);

  await page.getByRole("link", { name: "Create Post" }).click();
  assertUrl("create_post");

  await page.getByRole("link", { name: "Create Community" }).click();
  assertUrl("create_community");

  await page.getByRole("link", { name: "Communities" }).click();
  assertUrl("communities");

  await page.getByRole("link", { name: "Search" }).click();
  assertUrl("search");

  await page.getByRole("link", { name: "Modlog" }).click();
  assertUrl("modlog");

  await page.getByRole("link", { name: "Instances" }).click();
  assertUrl("instances");

  await page.getByRole("link", { name: "Legal" }).click();
  assertUrl("legal");

  await page.getByRole("link", { name: "lemmy-dev" }).click();
  await page.waitForURL(`${baseURL}/`);
  assertUrl("");
});
