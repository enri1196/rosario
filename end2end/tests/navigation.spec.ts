import { expect, test, type Page } from "@playwright/test";

const APP_URL = "http://127.0.0.1:3000/";

function sectionNavigation(page: Page) {
  return page.getByRole("navigation", { name: "Sezioni dell'applicazione" });
}

test("renders only the default Guide panel and switches sections by pointer", async ({ page }) => {
  await page.goto(APP_URL);

  const navigation = sectionNavigation(page);
  const guideTab = navigation.getByRole("tab", { name: "Guida" });
  const mysteriesTab = navigation.getByRole("tab", { name: "Misteri" });
  const prayersTab = navigation.getByRole("tab", { name: "Preghiere" });

  await expect(navigation.getByRole("tab")).toHaveCount(3);
  await expect(guideTab).toHaveAttribute("aria-selected", "true");
  await expect(guideTab).toHaveAttribute("aria-controls", "guide-panel");
  await expect(guideTab).toHaveAttribute("tabindex", "0");
  await expect(mysteriesTab).toHaveAttribute("aria-selected", "false");
  await expect(prayersTab).toHaveAttribute("aria-selected", "false");
  await expect(page.getByRole("tabpanel", { name: "Guida" })).toBeVisible();
  await expect(page.getByRole("tabpanel")).toHaveCount(1);
  await expect(page.locator("#mysteries-panel, #prayers-panel")).toHaveCount(0);

  await mysteriesTab.click();
  await expect(mysteriesTab).toHaveAttribute("aria-selected", "true");
  await expect(page.getByRole("tabpanel", { name: "Misteri" })).toBeVisible();
  await expect(page.getByRole("heading", { name: "I venti Misteri" })).toBeFocused();
  await expect(page.locator("#guide-panel, #prayers-panel")).toHaveCount(0);

  await prayersTab.click();
  await expect(prayersTab).toHaveAttribute("aria-selected", "true");
  await expect(page.getByRole("tabpanel", { name: "Preghiere" })).toBeVisible();
  await expect(page.getByRole("heading", { name: "Preghiere del Rosario" })).toBeFocused();
  await expect(page.locator("#guide-panel, #mysteries-panel")).toHaveCount(0);
});

test("uses roving focus and manual keyboard activation", async ({ page }) => {
  await page.goto(APP_URL);

  const navigation = sectionNavigation(page);
  const guideTab = navigation.getByRole("tab", { name: "Guida" });
  const mysteriesTab = navigation.getByRole("tab", { name: "Misteri" });
  const prayersTab = navigation.getByRole("tab", { name: "Preghiere" });

  await guideTab.focus();
  await page.keyboard.press("ArrowRight");
  await expect(mysteriesTab).toBeFocused();
  await expect(guideTab).toHaveAttribute("aria-selected", "true");
  await page.keyboard.press("Enter");
  await expect(mysteriesTab).toHaveAttribute("aria-selected", "true");
  await expect(page.getByRole("heading", { name: "I venti Misteri" })).toBeFocused();

  await mysteriesTab.focus();
  await page.keyboard.press("End");
  await expect(prayersTab).toBeFocused();
  await expect(mysteriesTab).toHaveAttribute("aria-selected", "true");
  await page.keyboard.press("Space");
  await expect(prayersTab).toHaveAttribute("aria-selected", "true");

  await prayersTab.focus();
  await page.keyboard.press("Home");
  await expect(guideTab).toBeFocused();
  await page.keyboard.press("Enter");
  await expect(guideTab).toHaveAttribute("aria-selected", "true");
  await expect(page.getByRole("heading", { name: "Come recitare il Rosario" })).toBeFocused();
});

test("preserves an active guided session while another section is mounted", async ({ page }) => {
  await page.goto(APP_URL);
  await page.getByRole("button", { name: "Avvia il Rosario guidato" }).click();

  const guided = page.getByRole("region", { name: "Rosario guidato" });
  await expect(guided).toBeVisible();
  await expect(page.locator(".guide")).toHaveCount(0);
  await expect(page.getByRole("heading", { name: "Come recitare il Rosario" })).toHaveCount(0);
  await expect(page.locator(".prayer-intention")).toHaveCount(0);
  await expect(page.locator(".rosary-wrap, .mystery-recommendation, .steps-legend")).toHaveCount(0);
  await guided.getByRole("button", { name: "Avanti" }).click();
  await expect(guided.locator(".guided-progress")).toHaveText("Passo 2 di 31");

  await sectionNavigation(page).getByRole("tab", { name: "Misteri" }).click();
  await expect(guided).toHaveCount(0);
  await expect(page.getByRole("tabpanel", { name: "Misteri" })).toBeVisible();

  await sectionNavigation(page).getByRole("tab", { name: "Guida" }).click();
  await expect(guided).toBeVisible();
  await expect(guided.locator(".guided-progress")).toHaveText("Passo 2 di 31");
  await expect(guided.getByRole("heading", { level: 4 })).toBeFocused();

  await guided.getByRole("button", { name: "Chiudi il Rosario guidato" }).click();
  await expect(guided).toHaveCount(0);
  await expect(page.locator(".prayer-intention")).toBeVisible();
  await expect(page.locator(".rosary-wrap")).toBeVisible();
  await expect(page.getByRole("heading", { name: "Come recitare il Rosario" })).toBeFocused();
});

test("keeps the mobile tab row compact, translated, and free of page overflow", async ({ page }) => {
  await page.setViewportSize({ width: 390, height: 844 });
  await page.goto(APP_URL);

  const navigation = sectionNavigation(page);
  const tabs = navigation.getByRole("tab");
  await expect(tabs).toHaveCount(3);
  await expect(navigation).toHaveCSS("position", "sticky");

  for (const tab of await tabs.all()) {
    const box = await tab.boundingBox();
    expect(box).not.toBeNull();
    expect(box!.height).toBeGreaterThanOrEqual(48);
  }

  await navigation.getByRole("tab", { name: "Preghiere" }).click();
  await expect(page.getByRole("tabpanel", { name: "Preghiere" })).toBeVisible();
  expect(await page.evaluate(() => document.documentElement.scrollWidth)).toBeLessThanOrEqual(390);

  await page.getByLabel("Lingua").selectOption("en");
  const englishNavigation = page.getByRole("navigation", { name: "Application sections" });
  await expect(englishNavigation.getByRole("tab")).toHaveText(["Guide", "Mysteries", "Prayers"]);
  await expect(englishNavigation.getByRole("tab", { name: "Prayers" })).toHaveAttribute(
    "aria-selected",
    "true",
  );
});
