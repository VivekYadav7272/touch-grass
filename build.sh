wasm-pack build --target=web

rm -r ./extension/snippets/ || true
rm ./extension/touch_grass_bg.wasm || true
rm ./extension/touch_grass.js || true
rm ./extension/output.css || true

npm install -D tailwindcss

npx tailwindcss@3 -i ./src/extension_ui/input.css -o ./extension/output.css
cp -r ./pkg/snippets ./extension/snippets
cp ./pkg/touch_grass_bg.wasm ./extension/touch_grass_bg.wasm
cp ./pkg/touch_grass.js ./extension/touch_grass.js
