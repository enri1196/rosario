import { expect, test } from "@playwright/test";

const APP_URL = "http://127.0.0.1:3000/";

test("initial theme is valid and follows an unsaved system preference", async ({ page }) => {
  await page.emulateMedia({ colorScheme: "light" });
  await page.goto(APP_URL);

  await expect(page.locator("html")).toHaveAttribute("data-theme", "light");
  await expect
    .poll(() => page.evaluate(() => localStorage.getItem("rosary-theme")))
    .toBeNull();

  await page.emulateMedia({ colorScheme: "dark" });
  await page.reload();
  await expect(page.locator("html")).toHaveAttribute("data-theme", "dark");
});

test("theme toggle updates state without navigation and persists on reload", async ({ page }) => {
  const consoleErrors: string[] = [];
  page.on("console", (message) => {
    if (message.type() === "error") consoleErrors.push(message.text());
  });

  await page.emulateMedia({ colorScheme: "dark" });
  await page.goto(APP_URL);
  const originalUrl = page.url();
  const toggle = page.getByRole("button", { name: /Passa al tema chiaro/ });

  await expect(toggle).toHaveAttribute("aria-pressed", "false");
  await toggle.click();

  await expect(page.locator("html")).toHaveAttribute("data-theme", "light");
  await expect(page.getByRole("button", { name: /Passa al tema scuro/ })).toHaveAttribute(
    "aria-pressed",
    "true",
  );
  expect(page.url()).toBe(originalUrl);
  await expect.poll(() => page.evaluate(() => localStorage.getItem("rosary-theme"))).toBe("light");

  await page.reload();
  await expect(page.locator("html")).toHaveAttribute("data-theme", "light");
  expect(consoleErrors).toEqual([]);
});

test("theme control remains bilingual and keyboard operable", async ({ page }) => {
  await page.emulateMedia({ colorScheme: "dark" });
  await page.goto(APP_URL);

  await page.getByLabel("Lingua").selectOption("en");
  const toggle = page.getByRole("button", { name: /Theme: Switch to light theme/ });
  await expect(page.locator("html")).toHaveAttribute("lang", "en");

  await toggle.focus();
  await expect(toggle).toBeFocused();
  await page.keyboard.press("Enter");

  await expect(page.locator("html")).toHaveAttribute("data-theme", "light");
  await expect(page.getByRole("button", { name: /Theme: Switch to dark theme/ })).toBeVisible();
  await expect(page.getByRole("heading", { level: 1 })).toHaveText("Guide to the Rosary");
});

test("language and theme controls stay aligned with a square toggle", async ({ page }) => {
  await page.setViewportSize({ width: 390, height: 844 });
  await page.goto(APP_URL);

  const selectBox = await page.getByLabel("Lingua").boundingBox();
  const toggleBox = await page.getByRole("button", { name: /Tema:/ }).boundingBox();

  expect(selectBox).not.toBeNull();
  expect(toggleBox).not.toBeNull();
  expect(Math.abs(toggleBox!.width - toggleBox!.height)).toBeLessThanOrEqual(0.5);
  expect(Math.abs(toggleBox!.height - selectBox!.height)).toBeLessThanOrEqual(0.5);
  expect(Math.abs(toggleBox!.y - selectBox!.y)).toBeLessThanOrEqual(0.5);
});

test("key surfaces and the original gold rosary remain visible in both themes", async ({ page }) => {
  await page.emulateMedia({ colorScheme: "dark" });
  await page.goto(APP_URL);

  const rosaryGold = {
    dark: {
      chain: "rgb(121, 104, 68)",
      beadHighlight: "rgb(240, 221, 167)",
      beadShadow: "rgb(111, 89, 44)",
    },
    light: {
      chain: "rgb(102, 80, 36)",
      beadHighlight: "rgb(234, 219, 146)",
      beadShadow: "rgb(103, 80, 34)",
    },
  } as const;

  for (const theme of ["dark", "light"] as const) {
    await expect(page.locator("html")).toHaveAttribute("data-theme", theme);
    for (const selector of [".site-header", ".guide-box", ".rosary", ".footer"]) {
      await expect(page.locator(selector).first()).toBeVisible();
    }

    await page.getByRole("tab", { name: "Preghiere" }).click();
    await expect(page.locator(".prayers-section")).toBeVisible();
    await page.getByRole("tab", { name: "Misteri" }).click();
    await expect(page.locator(".mystery-card").first()).toBeVisible();
    await page.getByRole("tab", { name: "Guida" }).click();

    await expect(page.locator(".chain").first()).toHaveCSS("stroke", rosaryGold[theme].chain);
    await expect(page.locator("#bead stop").first()).toHaveCSS(
      "stop-color",
      rosaryGold[theme].beadHighlight,
    );
    await expect(page.locator("#bead stop").last()).toHaveCSS(
      "stop-color",
      rosaryGold[theme].beadShadow,
    );

    if (theme === "dark") {
      await page.getByRole("button", { name: /Passa al tema chiaro/ }).click();
    }
  }
});
