import { expect, test, type Locator, type Page } from "@playwright/test";

const APP_URL = "http://127.0.0.1:3000/";
const INTENTIONS_STORAGE_KEY = "rosary-intentions";

function intentionEditor(page: Page): Locator {
  return page.getByRole("region", { name: "Intenzione di preghiera" });
}

function guidedPrayer(page: Page): Locator {
  return page.getByRole("region", { name: "Rosario guidato" });
}

test("saves, reloads, edits, clears, and shares one private intention with guided prayer", async ({
  page,
}) => {
  await page.emulateMedia({ colorScheme: "dark" });
  await page.goto(APP_URL);

  const editor = intentionEditor(page);
  const textarea = editor.getByLabel("La tua intenzione");
  const privateIntention = "Per la mia famiglia e per la pace";
  const consoleMessages: string[] = [];
  const requestsAfterEntry: string[] = [];
  page.on("console", (message) => consoleMessages.push(message.text()));

  await expect(textarea).toHaveValue("");
  await expect(editor.locator(".intention-count")).toHaveText("Caratteri: 0/500");
  await expect
    .poll(() => page.evaluate((key) => localStorage.getItem(key), INTENTIONS_STORAGE_KEY))
    .toBeNull();

  page.on("request", (request) => {
    requestsAfterEntry.push(`${request.url()} ${request.postData() ?? ""}`);
  });
  await textarea.fill(`  ${privateIntention}  `);
  await editor.getByRole("button", { name: "Salva intenzione" }).click();

  await expect(textarea).toHaveValue(privateIntention);
  await expect(editor.locator(".intention-feedback")).toHaveText(
    "Intenzione salvata in questo browser.",
  );
  await expect
    .poll(() => page.evaluate((key) => localStorage.getItem(key), INTENTIONS_STORAGE_KEY))
    .toBe(privateIntention);
  expect(page.url()).toBe(APP_URL);
  expect(requestsAfterEntry.some((request) => request.includes(privateIntention))).toBe(false);
  expect(consoleMessages.some((message) => message.includes(privateIntention))).toBe(false);
  await expect
    .poll(() =>
      page.evaluate((value) => {
        return [...document.querySelectorAll("*")].some((element) =>
          [...element.attributes].some((attribute) => attribute.value.includes(value)),
        );
      }, privateIntention),
    )
    .toBe(false);

  await page.getByRole("button", { name: "Avvia il Rosario guidato" }).click();
  const guided = guidedPrayer(page);
  await expect(guided.locator(".guided-intention-text")).toHaveText(privateIntention);
  await guided.getByRole("button", { name: "Avanti" }).click();
  await expect(guided.locator(".guided-intention")).toHaveCount(0);

  for (let index = 0; index < 30; index += 1) {
    await guided.locator(".guided-primary-button").click();
  }
  await expect(guided.getByRole("heading", { level: 4 })).toHaveText("Rosario completato");
  await expect(guided.locator(".guided-intention-text")).toHaveText(privateIntention);

  await page.reload();
  const reloadedEditor = intentionEditor(page);
  await expect(reloadedEditor.getByLabel("La tua intenzione")).toHaveValue(privateIntention);
  await reloadedEditor.getByLabel("La tua intenzione").fill("Per una persona cara");
  await reloadedEditor.getByRole("button", { name: "Salva intenzione" }).click();
  await expect
    .poll(() => page.evaluate((key) => localStorage.getItem(key), INTENTIONS_STORAGE_KEY))
    .toBe("Per una persona cara");

  await reloadedEditor.getByRole("button", { name: "Cancella intenzione" }).click();
  await expect(reloadedEditor.getByLabel("La tua intenzione")).toHaveValue("");
  await expect(reloadedEditor.locator(".intention-feedback")).toHaveText(
    "Intenzione cancellata.",
  );
  await expect
    .poll(() => page.evaluate((key) => localStorage.getItem(key), INTENTIONS_STORAGE_KEY))
    .toBeNull();

  await page.getByRole("button", { name: "Avvia il Rosario guidato" }).click();
  await expect(guidedPrayer(page).locator(".guided-intention")).toHaveCount(0);
});

test("accepts 500 Unicode scalar values and rejects an over-limit edit", async ({ page }) => {
  await page.goto(APP_URL);

  const editor = intentionEditor(page);
  const textarea = editor.getByLabel("La tua intenzione");
  const accepted = "🙏".repeat(500);
  const overLimit = `${accepted}🙏`;

  await textarea.fill(accepted);
  await expect(editor.locator(".intention-count")).toHaveText("Caratteri: 500/500");
  await editor.getByRole("button", { name: "Salva intenzione" }).click();
  await expect(editor.locator(".intention-feedback")).toHaveText(
    "Intenzione salvata in questo browser.",
  );
  await expect
    .poll(() =>
      page.evaluate((key) => {
        const value = localStorage.getItem(key);
        return value === null ? null : Array.from(value).length;
      }, INTENTIONS_STORAGE_KEY),
    )
    .toBe(500);

  await textarea.fill(overLimit);
  await expect(editor.locator(".intention-count")).toHaveText("Caratteri: 501/500");
  await expect(textarea).toHaveAttribute("aria-invalid", "true");
  await editor.getByRole("button", { name: "Salva intenzione" }).click();
  await expect(editor.locator(".intention-feedback")).toHaveText(
    "L'intenzione non può superare 500 caratteri.",
  );
  await expect
    .poll(() =>
      page.evaluate((key) => {
        const value = localStorage.getItem(key);
        return value === null ? null : Array.from(value).length;
      }, INTENTIONS_STORAGE_KEY),
    )
    .toBe(500);
});

test("keeps the editor usable in English, light theme, and storage-disabled browsers", async ({
  page,
}) => {
  await page.addInitScript(() => {
    Storage.prototype.getItem = () => {
      throw new DOMException("Storage disabled", "SecurityError");
    };
    Storage.prototype.setItem = () => {
      throw new DOMException("Storage disabled", "SecurityError");
    };
    Storage.prototype.removeItem = () => {
      throw new DOMException("Storage disabled", "SecurityError");
    };
  });
  await page.emulateMedia({ colorScheme: "dark" });
  await page.setViewportSize({ width: 390, height: 844 });
  await page.goto(APP_URL);

  await page.getByLabel("Lingua").selectOption("en");
  await page.getByRole("button", { name: /Theme: Switch to light theme/ }).click();
  await expect(page.locator("html")).toHaveAttribute("data-theme", "light");

  const editor = page.getByRole("region", { name: "Prayer intention" });
  const textarea = editor.getByLabel("Your intention");
  await textarea.fill("For peace");
  await editor.getByRole("button", { name: "Save intention" }).click();
  await expect(editor.locator(".intention-feedback")).toHaveText(
    "The change is active on this page, but the browser did not allow it to be saved.",
  );

  await page.getByRole("button", { name: "Start guided Rosary" }).click();
  let guided = page.getByRole("region", { name: "Guided Rosary" });
  await expect(guided.locator(".guided-intention-text")).toHaveText("For peace");
  await expect(editor.locator("textarea")).toHaveCSS("color", /rgb/);

  await editor.getByRole("button", { name: "Clear intention" }).click();
  await expect(guided.locator(".guided-intention")).toHaveCount(0);
  await expect(editor.locator(".intention-feedback")).toHaveText(
    "The change is active on this page, but the browser did not allow it to be saved.",
  );

  const saveButton = await editor.getByRole("button", { name: "Save intention" }).boundingBox();
  const clearButton = await editor.getByRole("button", { name: "Clear intention" }).boundingBox();
  expect(saveButton).not.toBeNull();
  expect(clearButton).not.toBeNull();
  expect(saveButton!.height).toBeGreaterThanOrEqual(48);
  expect(clearButton!.height).toBeGreaterThanOrEqual(48);
});
