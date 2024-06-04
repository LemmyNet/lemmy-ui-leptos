import { Page, expect } from "@playwright/test";

export async function showHome(page: Page) {
  await page.goto("/");

  await expect(
    page.getByRole("link", { name: "Login", exact: true })
  ).toBeVisible();
}

export async function loginLogoutTest(page: Page) {
  const loginLink = page.getByRole("link", {
    name: "Login",
    exact: true,
  });

  await expect(loginLink).toBeVisible();

  await loginLink.click({ force: true });

  const loginButton = page.getByRole("button", { name: "Login", exact: true });

  await expect(loginButton).toBeVisible();

  await page.getByLabel("Username", { exact: true }).fill("lemmy");
  await page.getByLabel("Password", { exact: true }).fill("lemmylemmy");
  await loginButton.click({ force: true });

  const userDropdownSummary = page.getByLabel("Logged in user dropdown");

  await expect(userDropdownSummary).toBeVisible();
  await userDropdownSummary.click({ force: true });
  await page.getByRole("button", { name: "Logout", exact: true }).click();
}

export async function persistThemeTest(page: Page) {
  await page.goto("/");

  const root = page.getByRole("document");
  await expect(root).toHaveAttribute("data-theme");

  const themeButton = page.getByLabel("Theme");
  await expect(themeButton).toBeVisible();

  await themeButton.click();
  await page.getByRole("button", { name: "Dark", exact: true }).click();

  await expect(root).toHaveAttribute("data-theme", "dark");
  await page.reload();

  await expect(root).toHaveAttribute("data-theme", "dark");
}
