const { defineConfig } = require('@playwright/test');
module.exports = defineConfig({
 testDir: './tests/browser', workers: 1, retries: 0,
 outputDir: 'test-results', reporter: [['list'], ['json', {outputFile: 'test-results/report.json'}]],
 use: {baseURL: process.env.MINES_URL || 'http://127.0.0.1:39720', headless: true,
  launchOptions: process.env.PLAYWRIGHT_CHROMIUM_EXECUTABLE ? {executablePath: process.env.PLAYWRIGHT_CHROMIUM_EXECUTABLE} : {}},
});
