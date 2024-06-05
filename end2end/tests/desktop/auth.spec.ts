import { expect, test } from "@playwright/test";

test("login, logout multiple times", async ({ page }) => {
  await page.goto("/");

  for (let i = 3; i; --i) {
    const loginLink = page.getByRole("link", {
      name: "Login",
      exact: true,
    });

    await expect(loginLink).toBeVisible();

    await loginLink.click({ force: true });

    const loginButton = page.getByRole("button", {
      name: "Login",
      exact: true,
    });

    await expect(loginButton).toBeVisible();

    await page.getByLabel("Username", { exact: true }).fill("lemmy");
    await page.getByLabel("Password", { exact: true }).fill("lemmylemmy");
    await loginButton.click({ force: true });

    const userDropdownSummary = page.getByLabel("Logged in user dropdown");

    await expect(userDropdownSummary).toBeVisible();
    await userDropdownSummary.click({ force: true });
    await page.getByRole("button", { name: "Logout", exact: true }).click();
  }
});
