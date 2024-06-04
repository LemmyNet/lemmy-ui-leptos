import { test } from "@playwright/test";
import { loginLogoutTest, persistThemeTest, showHome } from "./common";

test("show home", async ({ page }) => {
  await showHome(page);
});

test("login, logout multiple times", async ({ page }) => {
  await page.goto("/");

  for (let i = 3; i; --i) {
    await loginLogoutTest(page);
  }
});

test("persist theme selection between sessions", async ({ page }) => {
  await persistThemeTest(page);
});
