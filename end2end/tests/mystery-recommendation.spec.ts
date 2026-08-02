import { test, expect, type Page } from "@playwright/test";

const appUrl = "http://localhost:3000/";

async function browserLocalDateInputValue(page: Page): Promise<string> {
  return page.evaluate(() => {
    const date = new Date();
    const year = date.getFullYear();
    const month = String(date.getMonth() + 1).padStart(2, "0");
    const day = String(date.getDate()).padStart(2, "0");
    return `${year}-${month}-${day}`;
  });
}

async function recommendationText(page: Page): Promise<string> {
  return page.locator(".recommendation-result").innerText();
}

test("starts on the browser-local current date and resets after exploring", async ({ page }) => {
  await page.goto(appUrl);

  const dateInput = page.getByLabel("Scegli una data");
  const today = await browserLocalDateInputValue(page);
  await expect(dateInput).toHaveValue(today);
  await expect(page.locator(".recommendation-date")).toContainText("Data selezionata");

  await dateInput.fill("2026-08-03");
  await expect(page.locator(".recommendation-mystery")).toContainText("Misteri Gaudiosi");
  await expect(page.locator(".recommendation-reason")).toContainText("giorno della settimana");

  await page.getByRole("button", { name: "Oggi" }).click();
  await expect(dateInput).toHaveValue(await browserLocalDateInputValue(page));
});

test("explains seasonal and feast recommendations", async ({ page }) => {
  await page.goto(appUrl);

  const dateInput = page.getByLabel("Scegli una data");
  await dateInput.fill("2026-12-01");
  await expect(page.locator(".recommendation-mystery")).toContainText("Misteri Gaudiosi");
  await expect(page.locator(".recommendation-reason")).toContainText("Avvento");

  await dateInput.fill("2026-09-14");
  await expect(page.locator(".recommendation-mystery")).toContainText("Misteri Dolorosi");
  await expect(page.locator(".recommendation-reason")).toContainText("celebrazione liturgica");
});

test("rejects invalid input without replacing the last valid recommendation", async ({ page }) => {
  await page.goto(appUrl);

  const dateInput = page.getByLabel("Scegli una data");
  await dateInput.fill("2026-08-03");
  const previousRecommendation = await recommendationText(page);

  await dateInput.evaluate((input: HTMLInputElement) => {
    input.value = "2026-02-30";
    input.dispatchEvent(new Event("input", { bubbles: true }));
  });

  await expect(dateInput).toHaveAttribute("aria-invalid", "true");
  await expect(page.getByRole("alert")).toContainText("Inserisci una data valida");
  await expect.poll(() => recommendationText(page)).toBe(previousRecommendation);
});

test("date exploration is fully localized in English", async ({ page }) => {
  await page.goto(appUrl);
  await page.getByLabel("Lingua").selectOption("en");

  const dateInput = page.getByLabel("Choose a date");
  await expect(page.getByText("Explore the Mysteries recommended for another day.")).toBeVisible();

  await dateInput.fill("2026-04-05");
  await expect(page.locator(".recommendation-mystery")).toContainText("Glorious Mysteries");
  await expect(page.locator(".recommendation-reason")).toContainText("Easter season");

  await dateInput.fill("");
  await expect(page.getByRole("alert")).toContainText("Enter a valid date");
  await page.getByRole("button", { name: "Today" }).click();
  await expect(dateInput).toHaveValue(await browserLocalDateInputValue(page));
  await expect(page.getByRole("alert")).toHaveCount(0);
});
