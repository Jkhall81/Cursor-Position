# Cursor-Position

This is a small program used to find the exact positions of things on your screen. I use when I am creating new browser automation tools. Mostly with browsers. When I need the coordinates of inputs or buttons on a webpage, I use this tool to find the position of those elements. Once the program has started you can move your mouse anywhere on the screen and when you click somewhere, the X and Y coordinates of that position will appear in the terminal. There are lots of other tools out there that can do this, but I wanted something small that I don't have to install that will do this for me.

## How to Setup

clone this repo. If you don't have rust on your machine you will need to install that. CD into the repo. Excecute the command 'cargo build'. Now you can either run the command cargo run from the root of the repo, or go to target/debug/ and click on the cursor_position.exe (in windows). This will work on linux as well.

## Use

run the program. After the program starts you need to hit enter for the program to start watching for clicks and displaying coordinates. So if you need to move some windows around or do something else, you can do that without having the program watch for and display coordinates. Also, if you want to pause the program you can hit enter. Enter will resume the program once it has been paused.
