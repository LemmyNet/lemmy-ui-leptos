import { Page, expect } from "@playwright/test";

export async function showHome(page: Page) {
  await page.goto("/");

  await expect(
    page.locator("a").getByText("Login", { exact: true }).first(),
  ).toHaveText("Login");
}

export async function loginLogoutTest(page: Page) {
  await expect(
    page.locator("a").getByText("Login", { exact: true }).first(),
  ).toHaveText("Login");

  await page.locator("a").getByText("Login", { exact: true }).first().click();

  await expect(page.getByRole("button").getByText("Login").first()).toHaveText(
    "Login",
  );

  await page.getByLabel("Username", { exact: true }).fill("lemmy");
  await page.getByLabel("Password", { exact: true }).fill("lemmylemmy");
  await page.waitForTimeout(1000);
  await page.getByRole("button").getByText("Login").click();

  await expect(page.locator("summary").getByText("lemmy")).toBeVisible();

  await page.locator("summary").getByText("lemmy").click();
  await page.getByRole("button").getByText("Logout", { exact: true }).click();
}

export async function persistThemeTest(page: Page) {
  await page.goto("/");

  await expect(page.locator("html")).toHaveAttribute("data-theme");

  await expect(
    page.locator("summary").getByText("Theme", { exact: true }),
  ).toBeVisible();

  await page.locator("summary").getByText("Theme", { exact: true }).click();
  await page.getByRole("button").getByText("Dark", { exact: true }).click();

  await expect(page.locator("html")).toHaveAttribute("data-theme", "dark");

  await page.goto("https://join-lemmy.org");
  await page.goto("/");

  await expect(page.locator("html")).toHaveAttribute("data-theme", "dark");
}
