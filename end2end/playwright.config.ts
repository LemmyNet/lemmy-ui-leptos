import {
  devices,
  defineConfig,
  Project,
  PlaywrightTestOptions,
  PlaywrightWorkerOptions,
} from "@playwright/test";

type DeviceDescription = (typeof devices)[string];

type ProjectDefinition = Project<
  PlaywrightTestOptions,
  PlaywrightWorkerOptions
>;

type UseOptions = ProjectDefinition["use"];
interface TestOptions {
  name: string;
  testMatch: string;
  use: UseOptions;
}

const createProject = ({
  name,
  testMatch,
  use,
}: TestOptions): ProjectDefinition => ({
  name,
  testMatch,
  use,
});

const createMobileProject = (name: string, use: UseOptions) =>
  createProject({ name, use, testMatch: "mobile/**" });

const createDesktopProject = (name: string, use: UseOptions) =>
  createProject({ name, use, testMatch: "desktop/**" });

export default defineConfig({
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
    trace: "on-first-retry",
    baseURL: "http://localhost:1237",
  },
  projects: [
    createDesktopProject("Chromium SSR", {
      ...devices["Desktop Chrome"],
      javaScriptEnabled: false,
    }),
    createDesktopProject("Chromium Hydrate", devices["Desktop Chrome"]),
    createDesktopProject("Firefox SSR", {
      ...devices["Desktop Firefox"],
      javaScriptEnabled: false,
    }),
    createDesktopProject("Firefox Hydrate", devices["Desktop Firefox"]),
    createDesktopProject("Edge SSR", {
      ...devices["Desktop Edge"],
      javaScriptEnabled: false,
    }),
    createDesktopProject("Edge Hydrate", devices["Desktop Edge"]),
    // createMobileProject("Galaxy S9+ SSR", {
    //   ...devices["Galaxy S9+"],
    //   javaScriptEnabled: false,
    // }),
    // createMobileProject("Galaxy S9+ Hydrate", devices["Galaxy S9+"]),
    // createMobileProject("Pixel 7 SSr", {
    //   ...devices["Pixel 7"],
    //   javaScriptEnabled: false,
    // }),
    // createMobileProject("Pixel 7 Hydrate", devices["Pixel 7"]),
  ],
});
