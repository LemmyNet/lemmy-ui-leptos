import type { PlaywrightTestConfig } from "@playwright/test";
import { devices } from "@playwright/test";

const config: PlaywrightTestConfig = {
  testDir: "./tests",
  timeout: 60 * 1000,
  expect: {
    timeout: 5000,
  },
  fullyParallel: true,
  forbidOnly: !!process.env.CI,
  retries: process.env.CI ? 2 : 0,
  // only 1 worker because server can only handle login tests
  // one at a time from the same account due to token clashes
  // another solution is to use multiple account fixture data
  workers: 1,
  reporter: [["html", { open: "never" }]],
  use: {
    actionTimeout: 0,
    trace: "on-first-retry",
  },

  projects: [
    {
      name: "Chromium SSR",
      testMatch: "ssr.spec.ts",
      use: {
        ...{
          javaScriptEnabled: false,
          baseURL: "http://localhost:1237",
        },
        ...devices["Desktop Chrome"],
      },
    },

    {
      name: "Chromium Hydrate",
      testMatch: "hydrate.spec.ts",
      use: {
        ...{
          // javaScriptEnabled: true,
          baseURL: "http://localhost:1237",
        },
        ...devices["Desktop Chrome"],
      },
    },

    {
      name: "Chromium CSR",
      testMatch: "csr.spec.ts",
      use: {
        ...{
          // deliberately trigger failure
          // javaScriptEnabled: false,
          baseURL: "http://localhost:1237",
        },
        ...devices["Desktop Chrome"],
      },
    },

    // Firefox performance very erratic with JS enabled
    // {
    //   name: "Firefox Hydrate",
    //   testMatch: "hydrate.spec.ts",
    //   use: {
    //     ...{
    //       // javaScriptEnabled: true,
    //       baseURL: 'http://localhost:1237',
    //     },
    //     ...devices["Desktop Firefox"],
    //   },
    // },

    // {
    //   name: "Firefox CSR",
    //   testMatch: "csr.spec.ts",
    //   use: {
    //     ...{
    //       // deliberately trigger failure
    //       // javaScriptEnabled: false,
    //       baseURL: 'http://localhost:1237',
    //     },
    //     ...devices["Desktop Firefox"],
    //   },
    // },

    {
      name: "Firefox SSR",
      testMatch: "ssr.spec.ts",
      use: {
        ...{
          javaScriptEnabled: false,
          baseURL: "http://localhost:1237",
        },
        ...devices["Desktop Firefox"],
      },
    },

    /* Test against other browsers. */
    // {
    //   name: "firefox",
    //   use: {
    //     ...devices["Desktop Firefox"],
    //   },
    // },

    // {
    //   name: "webkit",
    //   use: {
    //     ...devices["Desktop Safari"],
    //   },
    // },

    /* Test against mobile viewports. */
    // {
    //   name: 'Mobile Chrome',
    //   use: {
    //     ...devices['Pixel 5'],
    //   },
    // },
    // {
    //   name: 'Mobile Safari',
    //   use: {
    //     ...devices['iPhone 12'],
    //   },
    // },

    /* Test against branded browsers. */
    // {
    //   name: 'Microsoft Edge',
    //   use: {
    //     channel: 'msedge',
    //   },
    // },
    // {
    //   name: 'Google Chrome',
    //   use: {
    //     channel: 'chrome',
    //   },
    // },
  ],
};

export default config;
