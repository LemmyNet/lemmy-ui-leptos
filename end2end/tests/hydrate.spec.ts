import { test, expect } from "@playwright/test";
import {
  loginLogoutTest,
  persistLanguageTest,
  persistThemeTest,
} from "./common";

test("show home, login, logout multiple times", async ({ page }) => {
  await page.goto("/");
  let iterations = 3;
  while (iterations--) {
    await loginLogoutTest({ page });
  }
});

test("persist language selection between sessions", async ({ page }) => {
  await persistLanguageTest({ page });
});

test("persist theme selection between sessions", async ({ page }) => {
  await persistThemeTest({ page });
});
