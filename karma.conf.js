const { chromium } = require("playwright");
const path = require("path");

module.exports = function (config) {
  config.set({
    basePath: "",
    frameworks: ["jasmine", "@angular-devkit/build-angular"],
    plugins: [
      require("karma-jasmine"),
      require("karma-chrome-launcher"),
      require("karma-jasmine-html-reporter"),
      require("karma-coverage"),
      require("@angular-devkit/build-angular/plugins/karma"),
    ],
    client: {
      jasmine: {},
      clearContext: false,
    },
    coverageReporter: {
      dir: path.join(__dirname, "./coverage/finalshelf"),
      subdir: ".",
      reporters: [{ type: "html" }, { type: "text-summary" }],
    },
    reporters: ["progress", "kjhtml"],
    port: 9876,
    colors: true,
    logLevel: config.LOG_INFO,
    autoWatch: false,
    browsers: ["ChromiumHeadlessPlaywright"],
    singleRun: true,
    restartOnFileChange: false,
    customLaunchers: {
      ChromiumHeadlessPlaywright: {
        base: "ChromiumHeadless",
        flags: ["--no-sandbox", "--disable-gpu"],
        executablePath: chromium.executablePath(),
      },
    },
  });
};
