import { test, expect } from "@playwright/test";

test("homepage renders the rosary guide and switches language", async ({ page }) => {
  await page.goto("http://localhost:3000/");

  await expect(page).toHaveTitle("Guida al Rosario");
  await expect(page.getByRole("heading", { level: 1 })).toHaveText("Guida al Rosario");
  await expect(page.locator(".mystery-card")).toHaveCount(20);
  await expect(page.locator(".hail-mary-bead")).toHaveCount(50);
  // Five decade beads plus the initial Our Father on the pendant.
  await expect(page.locator(".our-father-bead")).toHaveCount(6);

  await page.getByLabel("Lingua").selectOption("en");
  await expect(page).toHaveTitle("Guide to the Rosary");
  await expect(page.getByRole("heading", { level: 1 })).toHaveText("Guide to the Rosary");
  await expect(page.locator("html")).toHaveAttribute("lang", "en");
});
