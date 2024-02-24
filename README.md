# TouchGrass (WIP)
A web extension (written in Rust btw) that you can use to time your YouTube usage and to control under what hours can you "doom-scroll" it.
The current goals of this project is to only disable the home page of YouTube, since that's where the most distraction happens. Allowing you to search will always be open.

Currently, it's only tested to work on Firefox, though Chromium support should be trivial.

# Goals
1) Get till the proof-of-concept stage (add the ability to lock YouTube homepage for a certain duration during the day, and settings for which is only changeable by a password) [IN PROGRESS]
2) Implement basic usage statistics. [IN PROGRESS]
3) Write enough Rust in there so that JavaScript doesn't appear as the largest part of the project 😭. [IN PROGRESS]

# Would be nice to have
1) Categorically remove videos from appearing on your homepage (maybe only blacklist gaming videos, or only whitelist development related videos?)


# How to build and run
1) Clone the repo.
2) Go to the root of the project (where Cargo.toml resides), and run 
`./build.sh`. You might need to give it execution privileges (`chmod +x ./build.sh`).
3) Compile the Tailwind CSS file into the extension/ folder as output.css. I do it by the Tailwind watcher command.
   (`npx tailwindcss -i ./src/extension_ui/input.css -o ./extension/output.css --watch`)
4) In case of Firefox, open the browser and go to `about:debugging`, and under the "This Firefox" tab on the left-hand panel, click on "Load Temporary Add-on".
5) A new dialog box should appear. There, select extension/manifest.json (really any file within extension/ folder should work), and select okay or whatever it says I can't be arsed to open that dialog box again.
6) That's it! Whatever skeleton of the project is written till that time should appear as a new extension. (It might be hidden under the "puzzle"/"plugin" icon in your menu bar).

Any contributions are welcome! (help me reorganise this project or write a build script pls)