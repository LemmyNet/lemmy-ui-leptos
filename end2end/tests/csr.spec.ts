import { test } from "@playwright/test";
import {
  loginLogoutTest,
  persistLanguageTest,
  persistThemeTest,
  showHome,
} from "./common";

test.beforeEach(async ({ context }) => {
  await context.route("*/**/serverfn/*", async (route) => {
    await route.abort();
    throw "CSR - error - server function request";
  });
});

test("show home", async ({ page }) => {
  await showHome({ page });
});

// doesn't trigger site info cache bug because of interceptor above
test("login, logout multiple times", async ({ page }) => {
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
