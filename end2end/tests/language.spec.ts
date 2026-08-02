import { test, expect } from "@playwright/test";

const appUrl = "http://localhost:3000/";
const languageStorageKey = "rosary-language";

test("defaults to Italian when no language preference is saved", async ({ page }) => {
  await page.goto(appUrl);

  await expect(page.getByLabel("Lingua")).toHaveValue("it");
  await expect(page).toHaveTitle("Guida al Rosario");
  await expect(page.locator("html")).toHaveAttribute("lang", "it");
  await expect
    .poll(() => page.evaluate((key) => localStorage.getItem(key), languageStorageKey))
    .toBeNull();
});

test("persists explicit English and Italian selections across reloads", async ({ page }) => {
  await page.goto(appUrl);

  await page.getByLabel("Lingua").selectOption("en");
  await expect(page).toHaveTitle("Guide to the Rosary");
  await expect(page.getByRole("heading", { level: 1 })).toHaveText("Guide to the Rosary");
  await expect(page.locator("html")).toHaveAttribute("lang", "en");
  await expect
    .poll(() => page.evaluate((key) => localStorage.getItem(key), languageStorageKey))
    .toBe("en");

  await page.reload();
  await expect(page.getByLabel("Language")).toHaveValue("en");
  await expect(page).toHaveTitle("Guide to the Rosary");
  await expect(page.locator("html")).toHaveAttribute("lang", "en");

  await page.getByLabel("Language").selectOption("it");
  await expect(page).toHaveTitle("Guida al Rosario");
  await expect(page.locator("html")).toHaveAttribute("lang", "it");
  await expect
    .poll(() => page.evaluate((key) => localStorage.getItem(key), languageStorageKey))
    .toBe("it");

  await page.reload();
  await expect(page.getByLabel("Lingua")).toHaveValue("it");
  await expect(page.locator("html")).toHaveAttribute("lang", "it");
});

test("falls back safely when the saved language code is invalid", async ({ page }) => {
  await page.goto(appUrl);
  await page.evaluate(
    ([key, value]) => localStorage.setItem(key, value),
    [languageStorageKey, "fr"],
  );

  await page.reload();

  await expect(page.getByLabel("Lingua")).toHaveValue("it");
  await expect(page).toHaveTitle("Guida al Rosario");
  await expect(page.locator("html")).toHaveAttribute("lang", "it");
  await expect
    .poll(() => page.evaluate((key) => localStorage.getItem(key), languageStorageKey))
    .toBe("fr");
});

test("keeps language switching usable when browser storage throws", async ({ page }) => {
  await page.addInitScript(() => {
    Storage.prototype.getItem = () => {
      throw new DOMException("Storage disabled", "SecurityError");
    };
    Storage.prototype.setItem = () => {
      throw new DOMException("Storage disabled", "SecurityError");
    };
  });

  await page.goto(appUrl);
  await expect(page).toHaveTitle("Guida al Rosario");

  await page.getByLabel("Lingua").selectOption("en");
  await expect(page).toHaveTitle("Guide to the Rosary");
  await expect(page.locator("html")).toHaveAttribute("lang", "en");
});
