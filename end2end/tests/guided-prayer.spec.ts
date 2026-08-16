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
  const close = guided.getByRole("button", { name: "Chiudi il Rosario guidato" });
  await expect(close.locator("svg")).toHaveCount(1);
  await expect(close).toHaveAttribute("title", "Chiudi il Rosario guidato");
  await expect(guided.locator(".guided-restart-button svg")).toHaveCount(1);
  await expect(guided.locator(".guided-restart-button")).toHaveAttribute("title", "Ricomincia");
  await expect(guided.getByRole("heading", { level: 4 })).toHaveText("Segno della Croce");
  await expect(guided.locator(".guided-progress")).toHaveText("Passo 1 di 31");
  await expect(guided.getByRole("button", { name: "Indietro" })).toBeDisabled();
  await expect(guided.getByRole("button", { name: "Avanti" })).toBeEnabled();
  await expect(guided.locator(".guided-step-panel-content")).toHaveCSS("text-align", "center");
  await expect(guided.locator(".guided-step-panel")).toHaveAttribute("tabindex", "0");
  await expect(guided.getByRole("button", { name: "Indietro" }).locator("svg")).toHaveCount(1);
  await expect(guided.getByRole("button", { name: "Avanti" }).locator("svg")).toHaveCount(1);

  const [previousBox, panelBox, nextBox] = await Promise.all([
    guided.getByRole("button", { name: "Indietro" }).boundingBox(),
    guided.locator(".guided-step-panel").boundingBox(),
    guided.getByRole("button", { name: "Avanti" }).boundingBox(),
  ]);
  expect(previousBox).not.toBeNull();
  expect(panelBox).not.toBeNull();
  expect(nextBox).not.toBeNull();
  expect(previousBox!.y + previousBox!.height / 2).toBeCloseTo(panelBox!.y + panelBox!.height / 2, 0);
  expect(nextBox!.y + nextBox!.height / 2).toBeCloseTo(panelBox!.y + panelBox!.height / 2, 0);

  await guided.getByRole("button", { name: "Avanti" }).focus();
  await page.keyboard.press("Enter");
  await expect(guided.getByRole("heading", { level: 4 })).toHaveText("Credo degli Apostoli");
  await expect(guided.getByRole("heading", { level: 4 })).toBeFocused();
  await expect(guided.locator(".guided-prayer-text")).toContainText("Credo in Dio");

  await guided.getByRole("button", { name: "Indietro" }).focus();
  await page.keyboard.press("Enter");
  await expect(guided.locator(".guided-progress")).toHaveText("Passo 1 di 31");
  await expect(guided.getByRole("heading", { level: 4 })).toBeFocused();
  await guided.getByRole("button", { name: "Avanti" }).click();
  await expect(guided.locator(".guided-progress")).toHaveText("Passo 2 di 31");
  await guided.locator(".guided-restart-button").focus();
  await page.keyboard.press("Enter");
  await expect(guided.locator(".guided-progress")).toHaveText("Passo 1 di 31");
  await expect(guided.getByRole("heading", { level: 4 })).toBeFocused();

  await guided.getByRole("button", { name: "Chiudi il Rosario guidato" }).focus();
  await page.keyboard.press("Enter");
  await expect(guided).toHaveCount(0);
  await expect(page.getByRole("heading", { name: "Come recitare il Rosario" })).toBeVisible();
});

test("lists the Creed first and Eternal Rest last in the Prayers view", async ({ page }) => {
  await page.goto(APP_URL);

  await page.getByRole("tab", { name: "Preghiere" }).click();
  const prayers = page.getByRole("tabpanel", { name: "Preghiere" });
  await expect(prayers.getByRole("heading", { level: 3 })).toHaveText([
    "Credo degli Apostoli",
    "Padre Nostro",
    "Ave Maria",
    "Gloria al Padre",
    "Preghiera di Fatima",
    "Salve Regina",
    "L'Eterno Riposo",
  ]);
  await expect(prayers).toContainText("L'eterno riposo dona loro, o Signore");

  await page.getByLabel("Lingua").selectOption("en");
  const englishPrayers = page.getByRole("tabpanel", { name: "Prayers" });
  await expect(englishPrayers.getByRole("heading", { level: 3 })).toHaveText([
    "The Apostles' Creed",
    "Our Father",
    "Hail Mary",
    "Glory Be",
    "Fatima Prayer",
    "Hail, Holy Queen",
    "Eternal Rest",
  ]);
  await expect(englishPrayers).toContainText("Eternal rest grant unto them, O Lord");
});

test("opens the shared session for a specific mystery card", async ({ page }) => {
  await page.goto(APP_URL);
  await page.getByRole("tab", { name: "Misteri" }).click();

  const nativityCard = page.locator(".mystery-card").filter({
    has: page.getByRole("heading", { name: "La Natività" }),
  });
  const prayMystery = nativityCard.getByRole("button", {
    name: "Prega questo Mistero: La Natività",
  });

  await expect(prayMystery).toBeVisible();
  await prayMystery.click();

  const guided = guidedPrayer(page);
  await expect(page.getByRole("tab", { name: "Guida" })).toHaveAttribute("aria-selected", "true");
  await expect(guided).toBeVisible();
  await expect(page.locator(".rosary-wrap, .mystery-recommendation, .steps-legend")).toHaveCount(0);
  await expect(guided.locator(".guided-selected-mystery")).toContainText("La Natività");
  await expect(guided).toContainText("Misteri Gaudiosi");
  await expect(guided.getByRole("heading", { level: 4 })).toHaveText("Segno della Croce");
  await expect(guided.getByRole("heading", { level: 4 })).toBeFocused();
});

test("reports decade progress, completes, and restarts from the primary action", async ({ page }) => {
  await page.goto(APP_URL);
  await page.getByRole("button", { name: "Avvia il Rosario guidato" }).click();

  const guided = guidedPrayer(page);
  for (let index = 0; index < 5; index += 1) {
    await guided.locator(".guided-next-button").click();
  }
  await expect(guided.locator(".guided-progress")).toHaveText(
    "Passo 6 di 31 · Decina 1 di 5 · Preghiera 1 di 5",
  );

  for (let index = 0; index < 2; index += 1) {
    await guided.locator(".guided-next-button").click();
  }
  await expect(guided.locator(".guided-progress")).toHaveText(
    "Passo 8 di 31 · Decina 1 di 5 · Preghiera 3 di 5",
  );
  await expect(guided.getByRole("heading", { level: 4 })).toHaveText("Dieci Ave Maria");

  for (let index = 0; index < 3; index += 1) {
    await guided.locator(".guided-next-button").click();
  }
  await expect(guided.locator(".guided-progress")).toHaveText(
    "Passo 11 di 31 · Decina 2 di 5 · Preghiera 1 di 5",
  );

  for (let index = 0; index < 21; index += 1) {
    await guided.locator(".guided-next-button").click();
  }
  await expect(guided.getByRole("heading", { level: 4 })).toHaveText("Rosario completato");
  await expect(guided.getByRole("heading", { level: 4 })).toBeFocused();
  await expect(guided).toContainText("Hai completato il Rosario");
  await expect(guided.getByRole("heading", { level: 5 })).toHaveText(
    "Preghiere facoltative dopo il Rosario",
  );
  await expect(guided).toContainText("Tre L'Eterno Riposo per le anime del Purgatorio");
  await expect(guided).toContainText("L'eterno riposo dona loro, o Signore");
  await expect(guided).toContainText("Per le intenzioni del Santo Padre");
  await expect(guided).toContainText("Angelo di Dio");
  await expect(guided).toContainText("Nel nome del Padre e del Figlio");

  const prayAgain = guided.getByRole("button", { name: "Prega di nuovo" });
  await expect(prayAgain).toBeVisible();
  await prayAgain.focus();
  await page.keyboard.press("Enter");
  await expect(guided.locator(".guided-progress")).toHaveText("Passo 1 di 31");
  await expect(guided.getByRole("button", { name: "Indietro" })).toBeDisabled();
  await expect(guided.getByRole("heading", { level: 4 })).toBeFocused();
});

test("uses a full-width mobile panel and labeled bottom controls without overflow", async ({ page }) => {
  await page.setViewportSize({ width: 390, height: 620 });
  await page.goto(APP_URL);

  const wrapBox = await page.locator(".rosary-wrap").boundingBox();
  const diagramBox = await page.locator(".rosary").boundingBox();
  const startBox = await page.getByRole("button", { name: "Avvia il Rosario guidato" }).boundingBox();
  expect(wrapBox).not.toBeNull();
  expect(diagramBox).not.toBeNull();
  expect(startBox).not.toBeNull();
  expect(diagramBox!.width).toBeGreaterThan(400);
  expect(diagramBox!.width).toBeCloseTo(wrapBox!.width, 0);
  expect(diagramBox!.x + diagramBox!.width / 2).toBeCloseTo(wrapBox!.x + wrapBox!.width / 2, 0);
  expect(startBox!.width).toBeLessThan(diagramBox!.width / 2);

  await page.getByRole("button", { name: "Avvia il Rosario guidato" }).click();

  const guided = guidedPrayer(page);
  const panel = guided.locator(".guided-step-panel");
  const previous = guided.getByRole("button", { name: "Indietro" });
  const next = guided.getByRole("button", { name: "Avanti" });

  await expect(previous.locator(".guided-control-label")).toHaveText("Indietro");
  await expect(next.locator(".guided-control-label")).toHaveText("Avanti");

  const [panelBox, previousBox, nextBox] = await Promise.all([
    panel.boundingBox(),
    previous.boundingBox(),
    next.boundingBox(),
  ]);
  expect(panelBox).not.toBeNull();
  expect(previousBox).not.toBeNull();
  expect(nextBox).not.toBeNull();
  expect(previousBox!.y).toBeGreaterThanOrEqual(panelBox!.y + panelBox!.height);
  expect(nextBox!.y).toBeGreaterThanOrEqual(panelBox!.y + panelBox!.height);
  expect(previousBox!.height).toBeGreaterThanOrEqual(48);
  expect(nextBox!.height).toBeGreaterThanOrEqual(48);

  await next.click();
  await expect(guided.getByRole("heading", { level: 4 })).toHaveText("Credo degli Apostoli");
  await expect.poll(() => panel.evaluate((element) => element.scrollHeight > element.clientHeight)).toBe(true);
  await panel.evaluate((element) => {
    element.scrollTop = element.scrollHeight;
  });
  await expect.poll(() => panel.evaluate((element) => element.scrollTop)).toBeGreaterThan(0);

  await previous.click();
  await expect(guided.getByRole("heading", { level: 4 })).toHaveText("Segno della Croce");
  await expect.poll(() => panel.evaluate((element) => element.scrollTop)).toBe(0);
  expect(await page.evaluate(() => document.documentElement.scrollWidth)).toBeLessThanOrEqual(390);
});

test("keeps decade progress bilingual in dark and light themes", async ({ page }) => {
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
  for (let index = 0; index < 5; index += 1) {
    await guided.getByRole("button", { name: "Next" }).click();
  }
  await expect(guided.locator(".guided-progress")).toHaveText(
    "Step 6 of 31 · Decade 1 of 5 · Prayer 1 of 5",
  );
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

test("removes guided step motion when reduced motion is requested", async ({ page }) => {
  await page.emulateMedia({ reducedMotion: "reduce" });
  await page.goto(APP_URL);
  await page.getByRole("button", { name: "Avvia il Rosario guidato" }).click();

  const content = guidedPrayer(page).locator(".guided-step-panel-content");
  await expect(content).toHaveCSS("animation-name", "none");
  await guidedPrayer(page).getByRole("button", { name: "Avanti" }).click();
  await expect(content).toHaveCSS("animation-name", "none");
});
