# Omxserver #

## Overview ##
Minimal web server for playing video files on a Raspberry Pi using omxplayer

The "itch" this scratches is ability to hook a Raspberry Pi that hosts video files to a TV via HDMI. This server allows those files to be played back using a web interface. 

I have been using an Android application, Raspicast, to do this. However, there is no IOS version of it. Even if there was one, I want family members to be able to play back video on such devices without installing an application and just use a browser.

## Running ##

The server takes two parameters, the path to the file system root to serve and the http port to use.

Example:
~~~
cargo run /mnt/usb 8080
~~~

## Made Possible By ##
- https://github.com/tomaka/rouille
- https://github.com/djc/askama

## Near Term Enhancements ##

- A LOT of work on the HTML UI to look decent on mobile and desktop
- The wrapped omxplayer will attempt to play any file it finds whether or not it can, nice to handle this better.

## Someday Maybe ##
- Support slideshows of still photos using omxiv.


