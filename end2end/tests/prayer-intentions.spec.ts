import { expect, test, type Locator, type Page } from "@playwright/test";

const APP_URL = "http://127.0.0.1:3000/";
const INTENTIONS_STORAGE_KEY = "rosary-intentions";

function intentionEditor(page: Page): Locator {
  return page.getByRole("region", { name: "Intenzioni di preghiera" });
}

function guidedPrayer(page: Page): Locator {
  return page.getByRole("region", { name: "Rosario guidato" });
}

async function addIntention(editor: Locator, value: string): Promise<void> {
  await editor.getByRole("button", { name: "Aggiungi intenzione" }).click();
  const input = editor.getByLabel("Nuova intenzione");
  await input.fill(value);
  await input.press("Enter");
}

test("adds, reloads, and shares an ordered private tag list with guided prayer", async ({
  page,
}) => {
  await page.emulateMedia({ colorScheme: "dark" });
  await page.goto(APP_URL);

  const editor = intentionEditor(page);
  const input = editor.getByLabel("Nuova intenzione");
  const addButton = editor.getByRole("button", { name: "Aggiungi intenzione" });
  const privateIntentions = ["Per la mia famiglia", "Per la pace"];
  const consoleMessages: string[] = [];
  const requestsAfterEntry: string[] = [];
  page.on("console", (message) => consoleMessages.push(message.text()));

  await expect(input).toHaveCount(0);
  await expect(editor.locator(".intention-empty-state")).toHaveCount(0);
  await expect(editor.locator(".intention-total-count")).toHaveText("0/50");
  await expect(editor.locator(".intention-total-count")).toHaveAttribute(
    "aria-label",
    "Intenzioni: 0/50",
  );
  await expect(editor.locator(".intention-tags > li").last().getByRole("button")).toHaveAttribute(
    "aria-label",
    "Aggiungi intenzione",
  );
  await expect
    .poll(() => page.evaluate((key) => localStorage.getItem(key), INTENTIONS_STORAGE_KEY))
    .toBeNull();

  page.on("request", (request) => {
    requestsAfterEntry.push(`${request.url()} ${request.postData() ?? ""}`);
  });
  await addButton.click();
  await expect(input).toBeFocused();
  await expect(addButton).toBeDisabled();
  await editor.getByRole("heading", { name: "Intenzioni di preghiera" }).click();
  await expect(input).toHaveCount(0);
  await expect(editor.locator(".intention-tag")).toHaveCount(0);

  await addButton.click();
  await input.fill(`  ${privateIntentions[0]}  `);
  await editor.getByRole("heading", { name: "Intenzioni di preghiera" }).click();
  await addIntention(editor, privateIntentions[1]);

  await expect(input).toHaveCount(0);
  await expect(editor.locator(".intention-tag-text")).toHaveText(privateIntentions);
  await expect(editor.locator(".intention-feedback")).toHaveCount(0);
  await expect(editor.locator(".intention-total-count")).toHaveText("2/50");
  await expect
    .poll(() =>
      page.evaluate((key) => JSON.parse(localStorage.getItem(key) ?? "[]"), INTENTIONS_STORAGE_KEY),
    )
    .toEqual(privateIntentions);
  expect(page.url()).toBe(APP_URL);
  expect(requestsAfterEntry.some((request) => privateIntentions.some((value) => request.includes(value)))).toBe(false);
  expect(consoleMessages.some((message) => privateIntentions.some((value) => message.includes(value)))).toBe(false);
  await expect
    .poll(() =>
      page.evaluate((values) => {
        return [...document.querySelectorAll("*")].some((element) =>
          [...element.attributes].some((attribute) =>
            values.some((value) => attribute.value.includes(value)),
          ),
        );
      }, privateIntentions),
    )
    .toBe(false);

  await page.getByRole("button", { name: "Avvia il Rosario guidato" }).click();
  const guided = guidedPrayer(page);
  await expect(guided.locator(".guided-intention-tag")).toHaveText(privateIntentions);
  await guided.getByRole("button", { name: "Avanti" }).click();
  await expect(guided.locator(".guided-intentions")).toHaveCount(0);

  for (let index = 0; index < 30; index += 1) {
    await guided.locator(".guided-next-button").click();
  }
  await expect(guided.getByRole("heading", { level: 4 })).toHaveText("Rosario completato");
  await expect(guided.locator(".guided-intention-tag")).toHaveText(privateIntentions);

  await page.reload();
  const reloadedEditor = intentionEditor(page);
  await expect(reloadedEditor.getByLabel("Nuova intenzione")).toHaveCount(0);
  await expect(reloadedEditor.locator(".intention-tag-text")).toHaveText(privateIntentions);
});

test("enforces 50 Unicode characters, unique tags, and a maximum of 50 intentions", async ({
  page,
}) => {
  await page.goto(APP_URL);

  const editor = intentionEditor(page);
  const input = editor.getByLabel("Nuova intenzione");
  const addButton = editor.getByRole("button", { name: "Aggiungi intenzione" });
  const accepted = "🙏".repeat(50);

  await addButton.click();
  await input.fill(accepted);
  await expect(editor.locator(".intention-character-count")).toHaveText("50/50");
  await expect(editor.locator(".intention-character-count")).toHaveAttribute(
    "aria-label",
    "Caratteri: 50/50",
  );
  await input.press("Enter");
  await expect(editor.locator(".intention-feedback")).toHaveCount(0);

  await addButton.click();
  await input.fill(accepted);
  await input.press("Enter");
  await expect(editor.locator(".intention-feedback")).toHaveText(
    "Questa intenzione è già presente.",
  );

  await input.fill(`${accepted}🙏`);
  await expect(editor.locator(".intention-character-count")).toHaveText("51/50");
  await expect(editor.locator(".intention-character-count")).toHaveAttribute(
    "aria-label",
    "Caratteri: 51/50",
  );
  await expect(input).toHaveAttribute("aria-invalid", "true");
  await input.press("Enter");
  await expect(editor.locator(".intention-feedback")).toHaveText(
    "Ogni intenzione non può superare 50 caratteri.",
  );
  await input.press("Escape");

  for (let index = 1; index < 50; index += 1) {
    await addIntention(editor, `Intenzione ${index}`);
  }
  await expect(editor.locator(".intention-tag")).toHaveCount(50);
  await expect(editor.locator(".intention-total-count")).toHaveText("50/50");
  await expect(editor.locator(".intention-total-count")).toHaveAttribute(
    "aria-label",
    "Intenzioni: 50/50",
  );
  await expect(addButton).toBeDisabled();
  await expect
    .poll(() =>
      page.evaluate((key) => JSON.parse(localStorage.getItem(key) ?? "[]").length, INTENTIONS_STORAGE_KEY),
    )
    .toBe(50);
});

test("reorders tags with drag and keyboard, deletes with the right-side x, and persists order", async ({
  page,
}) => {
  await page.goto(APP_URL);
  const editor = intentionEditor(page);

  for (const intention of ["Prima", "Seconda", "Terza"]) {
    await addIntention(editor, intention);
  }

  let tags = editor.locator(".intention-tag");
  await tags.nth(0).dragTo(tags.nth(2));
  await expect(editor.locator(".intention-tag-text")).toHaveText(["Seconda", "Terza", "Prima"]);
  await expect(editor.locator(".intention-feedback")).toHaveCount(0);

  await tags.filter({ hasText: "Prima" }).focus();
  await page.keyboard.press("ArrowLeft");
  await expect(editor.locator(".intention-tag-text")).toHaveText(["Seconda", "Prima", "Terza"]);
  await expect(tags.filter({ hasText: "Prima" })).toBeFocused();

  const firstTag = tags.nth(0);
  const firstTextBox = await firstTag.locator(".intention-tag-text").boundingBox();
  const deleteBox = await firstTag.getByRole("button", { name: "Elimina intenzione 1" }).boundingBox();
  expect(firstTextBox).not.toBeNull();
  expect(deleteBox).not.toBeNull();
  expect(deleteBox!.x).toBeGreaterThan(firstTextBox!.x);
  await firstTag.getByRole("button", { name: "Elimina intenzione 1" }).click();
  await expect(editor.locator(".intention-tag-text")).toHaveText(["Prima", "Terza"]);
  await expect(editor.locator(".intention-feedback")).toHaveCount(0);
  await expect
    .poll(() =>
      page.evaluate((key) => JSON.parse(localStorage.getItem(key) ?? "[]"), INTENTIONS_STORAGE_KEY),
    )
    .toEqual(["Prima", "Terza"]);

  await page.reload();
  tags = intentionEditor(page).locator(".intention-tag");
  await expect(tags.locator(".intention-tag-text")).toHaveText(["Prima", "Terza"]);
});

test("migrates the previous single-intention storage value into one tag", async ({ page }) => {
  await page.goto(APP_URL);
  await page.evaluate(
    ([key, value]) => localStorage.setItem(key, value),
    [INTENTIONS_STORAGE_KEY, "  Per una persona cara  "],
  );

  await page.reload();

  await expect(intentionEditor(page).locator(".intention-tag-text")).toHaveText([
    "Per una persona cara",
  ]);
});

test("keeps tags usable in English, light theme, narrow screens, and disabled storage", async ({
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

  const editor = page.getByRole("region", { name: "Prayer intentions" });
  await editor.getByRole("button", { name: "Add intention" }).click();
  const input = editor.getByLabel("New intention");
  await input.fill("For peace");
  await expect(input).toHaveCSS("color", /rgb/);
  await input.press("Enter");
  await expect(editor.locator(".intention-feedback")).toHaveText(
    "The change is active on this page, but the browser did not allow it to be saved.",
  );
  await expect(editor.locator(".intention-tag-text")).toHaveText(["For peace"]);

  await page.getByRole("button", { name: "Start guided Rosary" }).click();
  const guided = page.getByRole("region", { name: "Guided Rosary" });
  await expect(guided.locator(".guided-intention-tag")).toHaveText(["For peace"]);
  await page.getByRole("button", { name: "Close guided Rosary" }).click();

  const tag = editor.locator(".intention-tag");
  const deleteButton = tag.getByRole("button", { name: "Delete intention 1" });
  const addBox = await editor.getByRole("button", { name: "Add intention" }).boundingBox();
  const deleteBox = await deleteButton.boundingBox();
  expect(addBox).not.toBeNull();
  expect(deleteBox).not.toBeNull();
  expect(addBox!.height).toBeGreaterThanOrEqual(48);
  expect(deleteBox!.height).toBeGreaterThanOrEqual(36);

  await deleteButton.click();
  await expect(editor.locator(".intention-tag")).toHaveCount(0);
  await expect(guided.locator(".guided-intentions")).toHaveCount(0);
  await expect(editor.locator(".intention-feedback")).toHaveText(
    "The change is active on this page, but the browser did not allow it to be saved.",
  );
});
