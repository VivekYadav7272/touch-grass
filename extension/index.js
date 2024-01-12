console.log("Hello from extension/index.js");

(async () => {
    const myModule = await import(browser.runtime.getURL("./touch_grass.js"));
    await myModule.default();
    setTimeout(() => myModule.touch_grass(), 2000);
})();