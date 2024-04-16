import { test } from "@playwright/test";
import {
  loginLogoutTest,
  persistLanguageTest,
  persistThemeTest,
  showHome,
} from "./common";

test("show home", async ({ page }) => {
  await showHome({ page });
});

test("login, logout multiple times", async ({ page }) => {
  await page.goto("/");
  let iterations = 3;
  while (iterations--) {
    console.log(`  SSR iteration ${3 - iterations}`);
    await loginLogoutTest({ page });
  }
});

test("persist language selection between sessions", async ({ page }) => {
  await persistLanguageTest({ page });
});

test("persist theme selection between sessions", async ({ page }) => {
  await persistThemeTest({ page });
});
