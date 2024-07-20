console.log("Hello from extension/index.js");

// Can't do the `import * as ...` thing cuz I'm not a top-level module here, I'm just a poor
// little injected script.
(async () => {
    const myModule = await import(browser.runtime.getURL("./touch_grass.js"));
    await myModule.default();
    setTimeout(() => myModule.touch_grass(), 2000);
})();