import { expect, test, type Page } from "@playwright/test";

const APP_URL = "http://127.0.0.1:3000/";
const CACHE_PREFIX = "rosary-shell-";
const CURRENT_CACHE = `${CACHE_PREFIX}v1`;

async function waitForActiveWorker(page: Page): Promise<void> {
  await page.evaluate(async () => {
    await navigator.serviceWorker.ready;
    if (!navigator.serviceWorker.controller) {
      await new Promise<void>((resolve) => {
        navigator.serviceWorker.addEventListener("controllerchange", () => resolve(), {
          once: true,
        });
      });
    }
  });
}

test("exposes valid install metadata, icons, and the root service worker", async ({
  page,
  request,
}) => {
  await page.goto(APP_URL);

  await expect(page.locator('link[rel="manifest"]')).toHaveAttribute(
    "href",
    "/manifest.webmanifest",
  );
  await expect(page.locator('link[rel="apple-touch-icon"]')).toHaveAttribute(
    "href",
    "/icons/rosary-192.png",
  );

  const manifestResponse = await request.get(`${APP_URL}manifest.webmanifest`);
  expect(manifestResponse.ok()).toBe(true);
  expect(manifestResponse.headers()["content-type"]).toContain("application/manifest+json");
  const manifest = await manifestResponse.json();
  expect(manifest).toMatchObject({
    name: "Rosary Guide · Guida al Rosario",
    short_name: "Rosario",
    start_url: "/",
    scope: "/",
    display: "standalone",
    background_color: "#0a0d14",
    theme_color: "#0a0d14",
  });
  expect(manifest.icons).toEqual(
    expect.arrayContaining([
      expect.objectContaining({ sizes: "192x192", type: "image/png", purpose: "any" }),
      expect.objectContaining({ sizes: "512x512", type: "image/png", purpose: "any" }),
      expect.objectContaining({ sizes: "192x192", type: "image/png", purpose: "maskable" }),
      expect.objectContaining({ sizes: "512x512", type: "image/png", purpose: "maskable" }),
    ]),
  );

  for (const icon of manifest.icons) {
    const iconResponse = await request.get(new URL(icon.src, APP_URL).toString());
    expect(iconResponse.ok()).toBe(true);
    expect(iconResponse.headers()["content-type"]).toContain("image/png");
  }

  const workerResponse = await request.get(`${APP_URL}service-worker.js`);
  expect(workerResponse.ok()).toBe(true);
  expect(workerResponse.headers()["content-type"]).toContain("javascript");
  expect(await workerResponse.text()).not.toContain("<!DOCTYPE html>");
});

test("reloads offline and keeps every client-side feature usable", async ({
  page,
  context,
  browserName,
}) => {
  test.skip(
    browserName !== "chromium",
    "Playwright offline emulation bypasses service-worker reloads in Firefox and WebKit.",
  );

  await page.goto(APP_URL);
  await waitForActiveWorker(page);

  await expect
    .poll(() =>
      page.evaluate(
        async (cacheName) => (await caches.open(cacheName)).match("/index.html").then(Boolean),
        CURRENT_CACHE,
      ),
    )
    .toBe(true);

  await context.setOffline(true);
  await page.reload({ waitUntil: "domcontentloaded" });
  await expect(page.getByRole("heading", { level: 1 })).toHaveText("Guida al Rosario");

  await page.getByLabel("Lingua").selectOption("en");
  await expect(page.getByRole("heading", { level: 1 })).toHaveText("Guide to the Rosary");

  const documentRoot = page.locator("html");
  const initialTheme = await documentRoot.getAttribute("data-theme");
  await page.getByRole("button", { name: /^Theme:/ }).click();
  await expect(documentRoot).toHaveAttribute(
    "data-theme",
    initialTheme === "light" ? "dark" : "light",
  );

  const dateInput = page.getByLabel("Choose a date (YYYY-MM-DD)");
  await dateInput.fill("2026-08-03");
  await expect(page.locator(".recommendation-mystery")).toContainText("Joyful Mysteries");

  const intentions = page.getByRole("region", { name: "Prayer intentions" });
  await intentions.getByRole("button", { name: "Add intention" }).click();
  await intentions.getByLabel("New intention").fill("For peace");
  await intentions.getByLabel("New intention").press("Enter");
  await expect(intentions.locator(".intention-tag-text")).toHaveText(["For peace"]);

  await page.getByRole("button", { name: "Start guided Rosary" }).click();
  const guided = page.getByRole("region", { name: "Guided Rosary" });
  await expect(guided.getByRole("heading", { level: 4 })).toHaveText("Sign of the Cross");
  await expect(guided.locator(".guided-intention-tag")).toHaveText(["For peace"]);

  await context.setOffline(false);
});

test("activation removes superseded Rosary shell caches", async ({ page }) => {
  await page.goto(APP_URL);
  await waitForActiveWorker(page);

  await page.evaluate(async (oldCache) => {
    await caches.open(oldCache);
  }, `${CACHE_PREFIX}v0`);

  await page.evaluate(async () => {
    const registration = await navigator.serviceWorker.register(
      "/service-worker.js?pwa-update-test=1",
      { scope: "/" },
    );
    const worker = registration.installing ?? registration.waiting ?? registration.active;
    if (worker && worker.state !== "activated") {
      await new Promise<void>((resolve) => {
        worker.addEventListener(
          "statechange",
          () => {
            if (worker.state === "activated") {
              resolve();
            }
          },
          { once: false },
        );
      });
    }
  });

  await expect
    .poll(() =>
      page.evaluate(async (prefix) => (await caches.keys()).filter((key) => key.startsWith(prefix)), CACHE_PREFIX),
    )
    .toEqual([CURRENT_CACHE]);
});
