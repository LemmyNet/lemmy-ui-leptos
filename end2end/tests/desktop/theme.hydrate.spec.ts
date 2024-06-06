import test, { expect } from "@playwright/test";

test("Can close theme dropdown by clicking outside", async ({ page }) => {
  page.goto("/");

  const themeButton = page.getByLabel("Theme");
  const themeDropdown = page.getByRole("group").filter({ has: themeButton });
  const themeDropdownList = themeDropdown.getByRole("list");

  await expect(themeDropdown).not.toHaveAttribute("open");
  await expect(themeDropdownList).not.toBeVisible();

  await themeButton.click();
  await expect(themeDropdown).toHaveAttribute("open");
  await expect(themeDropdownList).toBeVisible();

  await page.getByRole("document").click({ force: true });
  await expect(themeDropdown).not.toHaveAttribute("open");
  await expect(themeDropdownList).not.toBeVisible();
});
