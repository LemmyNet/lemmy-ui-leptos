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

const createProject = (
  name: string,
  testMatch: string,
  use: UseOptions
): ProjectDefinition => ({
  name,
  testMatch,
  use,
});

const createSsrProject = (name: string, device: DeviceDescription) =>
  createProject(name, "ssr.spec.ts", {
    ...device,
    javaScriptEnabled: false,
  });

const createHydrateProject = (name: string, device: DeviceDescription) =>
  createProject(name, "hydrate.spec.ts", device);

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
    createSsrProject("Chromium SSR", devices["Desktop Chrome"]),
    createHydrateProject("Chromium Hydrate", devices["Desktop Chrome"]),
    createSsrProject("Firefox SSR", devices["Desktop Firefox"]),
    createHydrateProject("Firefox Hydrate", devices["Desktop Firefox"]),
    createSsrProject("Edge SSR", devices["Desktop Edge"]),
    createHydrateProject("Edge Hydrate", devices["Desktop Edge"]),
  ],
});
