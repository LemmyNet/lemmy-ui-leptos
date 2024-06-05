import {
  devices,
  defineConfig,
  Project,
  PlaywrightTestOptions,
  PlaywrightWorkerOptions,
} from "@playwright/test";

type ProjectDefinition = Project<
  PlaywrightTestOptions,
  PlaywrightWorkerOptions
>;

type UseOptions = ProjectDefinition["use"];

const createProject = (
  name: string,
  use: UseOptions,
  screen: "desktop" | "mobile"
): ProjectDefinition => ({
  name,
  use,
  testMatch: screen === "desktop" ? "desktop/**" : "mobile/**",
});

const ssr = (def: ProjectDefinition): ProjectDefinition => ({
  ...def,
  use: {
    ...def.use!,
    javaScriptEnabled: false,
  },
  testIgnore: /.*hydrate.*/,
});

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
    createProject("Chromium Hydrate", devices["Desktop Chrome"], "desktop"),
    ssr(createProject("Chromium SSR", devices["Desktop Chrome"], "desktop")),
    createProject("Firefox Hydrate", devices["Desktop Firefox"], "desktop"),
    ssr(createProject("Firefox SSR", devices["Desktop Firefox"], "desktop")),
    createProject("Edge Hydrate", devices["Desktop Edge"], "desktop"),
    ssr(createProject("Edge Hydrate", devices["Desktop Edge"], "desktop")),
    createProject("Galaxy S9+ Hydrate", devices["Galaxy S9+"], "mobile"),
    ssr(createProject("Galaxy S9+ SSR", devices["Galaxy S9+"], "mobile")),
    createProject("Pixel 7 Hydrate", devices["Pixel 7"], "mobile"),
    ssr(createProject("Pixel 7 SSR", devices["Pixel 7"], "mobile")),
  ],
});
