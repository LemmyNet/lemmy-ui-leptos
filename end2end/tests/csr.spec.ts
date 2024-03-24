import { test, expect } from "@playwright/test";
import {
  loginLogoutTest,
  persistLanguageTest,
  persistThemeTest,
} from "./common";

test.beforeEach(async ({ context }) => {
  await context.route("*/**/serverfn/*", async (route) => {
    await route.abort();
    throw "CSR - Error - Server function request - " + route.toString();
  });
});

// doesn't fail on CSR because of interceptor above slows down logins
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
