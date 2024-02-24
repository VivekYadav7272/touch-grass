wasm-pack build --target=web
rm -r ./extension/snippets/
rm ./extension/touch_grass_bg.wasm
rm ./extension/touch_grass.js
cp -r ./pkg/snippets ./extension/snippets
cp ./pkg/touch_grass_bg.wasm ./extension/touch_grass_bg.wasm
cp ./pkg/touch_grass.js ./extension/touch_grass.js