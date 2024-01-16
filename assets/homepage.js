console.log("Hello from extension/homepage.js");

import * as myModule from "./touch_grass.js";

(async () => {
    await myModule.default("./touch_grass_bg.wasm");
    myModule.start_app();
})();