# Promodoro_App

My Pomodoro application is designed to keep track of a work break cycle established by the user. It keeps track of total work time each session and reports it to you below the timer display.

## Instructions for Build and Use

Steps to build and/or run the software:

1. Click on the desktop shortcut for the executable. 

Instructions for using the software:

1. Type in a positive whole number based on the number of minutes you plan to study for.
2. Type in a positive whole number based on the number of minutes you plan to take breaks for.
3. Press the start timer button.

## Development Environment

To recreate the development environment, you need the following software and/or libraries with the specified versions:

* Download visual studio code.
* Download rust 1.95.0 from https://rust-lang.org/tools/install/
* Enable the "rust" extension from the marketplace on vs code
* In the vscode terminal type "cargo new (file name)" to create a new cargo rust project.
* In the cargo.toml file add eframe = "0.27.2" to specify version of eframe crate.
* In the cargo.toml file add rodio = "0.19" to specify version of rodio crate.
* open terminal and type "cargo add eframe" to access the eframe crate.
* In the terminal type "cargo add rodio" to access the rodio crate.

## Useful Websites to Learn More

I found these websites useful in developing this software:

* [Claude.AI](https://chatgpt.com/)
* [ChatGPT.com](https://claude.ai/)
* [W3Schools](https://www.w3schools.com/)

## Future Work

The following items I plan to fix, improve, and/or add to this project in the future:

* [ ] Embed the audio file so it can be exported as part of the exe file.
* [ ] Add additional graphical representation of time.
* [ ] Hide input boxes while running the timer.
