# BuckleSpring Rust Reimplementation

## About

This project emulates the sound of the classic **IBM Model-M Space Saver bucklespring keyboard** while typing on your machine. The nostalgic sounds of each key press and release have been carefully sampled to deliver an authentic typing experience.

The Rust version uses the **`input` crate** (FFI bindings for libinput) to handle keyboard events and playback sounds for a realistic simulation of typing on an IBM Model-M.

![Model M](img/model-m.jpg)
![Buckle](img/buckle.gif)

## Why Rewrite in Rust?

The original project was written in C, but it is no longer supported on my machine running **Fedora 41 with GNOME**. I decided to clone the project and rewrite it entirely in **Rust** to replace the C code and make it compatible with modern systems.  

### Current Status:
- Fully working with **libinput**.
- **X11**, **macOS**, and **Windows** support are planned for future updates.

## Features

- Background process that plays **IBM Model-M** key sounds when typing.
- Supports key press and release events.


## Credits

- **Original Author**: [Ico Doornekamp](https://github.com/zevv/bucklespring)  
- **Rust Rewrite**: *[j03-dev](https::github.com/j03-dev/bucklespring-rust)*  

I respect the original work by Ico Doornekamp and thank him for the idea and inspiration for this reimplementation.


## Contributions

If anyone wants to contribute to support **X11**, **macOS**, or **Windows**, contributions are welcome!


**Enjoy the nostalgic sound of the IBM Model-M keyboard!**

