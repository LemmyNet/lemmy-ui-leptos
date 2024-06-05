import { expect, test } from "@playwright/test";

test("persist theme selection between sessions", async ({ page }) => {
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
});
