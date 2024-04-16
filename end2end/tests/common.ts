import { expect } from "@playwright/test";

export async function showHome({ page }) {
  await page.goto("/");

  await expect(
    page.locator("a").getByText("Login", { exact: true }).first(),
  ).toHaveText("Login");
}

export async function loginLogoutTest({ page }) {
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

  await expect(
    page.locator("summary").getByText("lemmy", { exact: true }),
  ).toBeVisible();

  await page.locator("summary").getByText("lemmy", { exact: true }).click();
  await page.getByRole("button").getByText("Logout", { exact: true }).click();
}

export async function persistLanguageTest({ page }) {
  await page.goto("/");

  await expect(page.locator("a").getByText("Modlog").first()).toHaveText(
    "Modlog",
  );

  await expect(
    page.locator("summary").getByText("Lang", { exact: true }),
  ).toBeVisible();

  await page.locator("summary").getByText("Lang", { exact: true }).click();
  await page.getByRole("button").getByText("FR", { exact: true }).click();

  await expect(page.locator("a").getByText("fr Modlog").first()).toHaveText(
    "fr Modlog",
  );

  await page.goto("https://join-lemmy.org");
  await page.goto("/");

  await expect(page.locator("a").getByText("fr Modlog").first()).toHaveText(
    "fr Modlog",
  );
}

export async function persistThemeTest({ page }) {
  await page.goto("/");

  await expect(page.locator("div[data-theme]")).toHaveAttribute("data-theme");

  await expect(
    page.locator("summary").getByText("Theme", { exact: true }),
  ).toBeVisible();

  await page.locator("summary").getByText("Theme", { exact: true }).click();
  await page.getByRole("button").getByText("Dark", { exact: true }).click();

  await expect(page.locator("div[data-theme]")).toHaveAttribute(
    "data-theme",
    "dark",
  );

  await page.goto("https://join-lemmy.org");
  await page.goto("/");

  await expect(page.locator("div[data-theme]")).toHaveAttribute(
    "data-theme",
    "dark",
  );
}
