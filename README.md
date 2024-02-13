# Side-Drop-CLI

## Overview
This code provides a simple generator and reader. By running the command `./qrt [file_path]`, you can quickly generate a QR Code in the terminal of the URL for your file. This is stored in [transfer.sh](https://transfer.sh/) with a limit to **1-day storage**, and only **1 visit** for the link. 

## Example
To demonstrate how the code works, here's an example:

```
./qrt wp.png
```

![QR Code Result](screenshot.png)

## Apple Shortcut for Enhanced Functionality
For enhanced functionality and quicker access, I've created an Apple Shortcut. You can find the shortcut [here](https://www.icloud.com/shortcuts/935b14647634407aafa36fcc081d08fa). 

### How it Works
This Apple Shortcut simplifies the process of scanning and downloading files using QR codes. It leverages the utility provided by this code to quickly scan QR codes and initiate download instead of having to visit the link embedded in the QR Code.

#### Demonstration
![Shortcut in Action](shortcut_action.gif)

## Android

Although there are no Shortcuts for Android, you can still scan the QR Code and download it from the original site.
