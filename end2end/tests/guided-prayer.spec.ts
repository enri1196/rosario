import { expect, test, type Locator, type Page } from "@playwright/test";

const APP_URL = "http://127.0.0.1:3000/";

function guidedPrayer(page: Page): Locator {
  return page.getByRole("region", { name: "Rosario guidato" });
}

test("opens from the guide and supports keyboard navigation, reset, and close", async ({ page }) => {
  await page.goto(APP_URL);

  const rosary = page.locator(".rosary-wrap");
  const start = rosary.getByRole("button", { name: "Avvia il Rosario guidato" });
  await expect(start).toBeVisible();
  await expect(rosary.locator(".rosary text")).toHaveCount(0);

  const [startBox, rosaryBox] = await Promise.all([
    start.boundingBox(),
    rosary.locator(".rosary").boundingBox(),
  ]);
  expect(startBox).not.toBeNull();
  expect(rosaryBox).not.toBeNull();
  expect(startBox!.x + startBox!.width / 2).toBeCloseTo(rosaryBox!.x + rosaryBox!.width / 2, 0);
  expect(startBox!.y + startBox!.height / 2).toBeCloseTo(rosaryBox!.y + rosaryBox!.height * 165 / 420, 0);

  await start.focus();
  await page.keyboard.press("Enter");

  const guided = guidedPrayer(page);
  await expect(guided).toBeVisible();
  await expect(guided.getByRole("button", { name: "Chiudi il Rosario guidato" })).toHaveText("×");
  await expect(guided.getByRole("heading", { level: 4 })).toHaveText("Segno della Croce");
  await expect(guided.locator(".guided-progress")).toHaveText("Passo 1 di 31");
  await expect(guided.getByRole("button", { name: "Indietro" })).toBeDisabled();
  await expect(guided.getByRole("button", { name: "Avanti" })).toBeEnabled();

  await guided.getByRole("button", { name: "Avanti" }).click();
  await expect(guided.getByRole("heading", { level: 4 })).toHaveText("Credo degli Apostoli");
  await expect(guided.getByRole("heading", { level: 4 })).toBeFocused();
  await expect(guided.locator(".guided-prayer-text")).toContainText("Credo in Dio");

  await guided.getByRole("button", { name: "Indietro" }).click();
  await expect(guided.locator(".guided-progress")).toHaveText("Passo 1 di 31");
  await guided.getByRole("button", { name: "Avanti" }).click();
  await expect(guided.locator(".guided-progress")).toHaveText("Passo 2 di 31");
  await guided.getByRole("button", { name: "Ricomincia" }).click();
  await expect(guided.locator(".guided-progress")).toHaveText("Passo 1 di 31");

  await guided.getByRole("button", { name: "Chiudi il Rosario guidato" }).click();
  await expect(guided).toHaveCount(0);
  await expect(page.getByRole("heading", { name: "Come recitare il Rosario" })).toBeVisible();
});

test("opens the shared session for a specific mystery card", async ({ page }) => {
  await page.goto(APP_URL);

  const nativityCard = page.locator(".mystery-card").filter({
    has: page.getByRole("heading", { name: "La Natività" }),
  });
  const prayMystery = nativityCard.getByRole("button", {
    name: "Prega questo Mistero: La Natività",
  });

  await expect(prayMystery).toBeVisible();
  await prayMystery.click();

  const guided = guidedPrayer(page);
  await expect(guided).toBeVisible();
  await expect(guided.locator(".guided-selected-mystery")).toContainText("La Natività");
  await expect(guided).toContainText("Misteri Gaudiosi");
  await expect(guided.getByRole("heading", { level: 4 })).toHaveText("Segno della Croce");
  await expect(guided.getByRole("heading", { level: 4 })).toBeFocused();
});

test("progresses through decades to completion and restarts", async ({ page }) => {
  await page.goto(APP_URL);
  await page.getByRole("button", { name: "Avvia il Rosario guidato" }).click();

  const guided = guidedPrayer(page);
  for (let index = 0; index < 5; index += 1) {
    await guided.locator(".guided-primary-button").click();
  }
  await expect(guided.locator(".guided-progress")).toHaveText("Passo 6 di 31");
  await expect(guided.locator(".guided-active-decade")).toContainText("Decina 1 di 5");

  for (let index = 0; index < 5; index += 1) {
    await guided.locator(".guided-primary-button").click();
  }
  await expect(guided.locator(".guided-progress")).toHaveText("Passo 11 di 31");
  await expect(guided.locator(".guided-active-decade")).toContainText("Decina 2 di 5");

  for (let index = 0; index < 21; index += 1) {
    await guided.locator(".guided-primary-button").click();
  }
  await expect(guided.getByRole("heading", { level: 4 })).toHaveText("Rosario completato");
  await expect(guided).toContainText("Hai completato il Rosario");

  await guided.getByRole("button", { name: "Prega di nuovo" }).click();
  await expect(guided.locator(".guided-progress")).toHaveText("Passo 1 di 31");
  await expect(guided.getByRole("button", { name: "Indietro" })).toBeDisabled();
});

test("keeps the guided flow bilingual in dark and light themes", async ({ page }) => {
  await page.setViewportSize({ width: 390, height: 844 });
  await page.emulateMedia({ colorScheme: "dark" });
  await page.goto(APP_URL);

  await page.getByRole("button", { name: "Avvia il Rosario guidato" }).click();
  await expect(guidedPrayer(page)).toHaveCSS("color", /rgb/);
  await guidedPrayer(page).getByRole("button", { name: "Chiudi il Rosario guidato" }).click();

  await page.getByLabel("Lingua").selectOption("en");
  await page.getByRole("button", { name: "Start guided Rosary" }).click();
  let guided = page.getByRole("region", { name: "Guided Rosary" });
  await expect(guided.locator(".guided-progress")).toHaveText("Step 1 of 31");
  await expect(guided.getByRole("heading", { level: 4 })).toHaveText("Sign of the Cross");
  await guided.getByRole("button", { name: "Close guided Rosary" }).click();

  await page.getByRole("button", { name: /Theme: Switch to light theme/ }).click();
  await expect(page.locator("html")).toHaveAttribute("data-theme", "light");
  await page.getByRole("button", { name: "Start guided Rosary" }).click();
  guided = page.getByRole("region", { name: "Guided Rosary" });
  await expect(guided.getByRole("button", { name: "Previous" })).toBeDisabled();
  await expect(guided.getByRole("button", { name: "Next" })).toBeEnabled();
  await guided.getByRole("button", { name: "Close guided Rosary" }).click();

  await page.getByLabel("Language").selectOption("it");
  await page.getByRole("button", { name: "Avvia il Rosario guidato" }).click();
  await expect(guidedPrayer(page).locator(".guided-progress")).toHaveText("Passo 1 di 31");
  const mobileNext = await guidedPrayer(page).getByRole("button", { name: "Avanti" }).boundingBox();
  expect(mobileNext).not.toBeNull();
  expect(mobileNext!.height).toBeGreaterThanOrEqual(48);
});
