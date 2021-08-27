Roo (🦘) means Reach Out and Open
---

This will find folders or files based on the search argument use in the settings.
Then after this is found it will call the command we want with it or its folder as argument.

Let's say you have this config in your .roo file:


```
walkdir: fd
delay: 750
commands:
  - key: "vs"
    command: "C:\\Program Files (x86)\\Microsoft Visual Studio\\2019\\Professional\\Common7\\IDE\\devenv.exe"
    command_type: "File"
    search_by: "Extension"
    search_for: "sln"
  - key: "cs"
    command: "C:\\Program Files\\Microsoft VS Code\\Code.exe"
    command_type: "Directory"
    search_by: "Extension"
    search_for: "sln"
```

Walkdir can be `walkdir` (which is one that I implemented but it's very slow) or `fd` which will use fd so it needs it installed

Delay is the delay to open the app in ms. Default is zero, but if you use virtual desktops and want to change to another one between asking the app to open and opening it you can add a delay in there.

If you run `roo vs` or `roo vs .`
It will search for all the files on that path that have the extension `.sln` and if there's more than one it will ask you which one you want opened, then it will open it on visual studio.

--- 
Troubleshoot
---

**Instead of roo cargo is installing roo-cli**
I'm hoping to have fixed this on v 0.0.5 of the app.

**Where to put the .roo file?**

To get to the home folder I'm using the `dirs-next` crate.
This is an example from their docs in case the username is Alice.
```
// Lin: Some(/home/alice)
// Win: Some(C:\Users\Alice)
// Mac: Some(/Users/Alice) 
```

**I can't run the app properly in mingw64**

If you want to run it in MINGW64 (Git bash) due to the selector when there are multiple files you won't be able to because the terminal type is a `FILE_TYPE_PIPE` instead of a `FILE_TYPE_CHAR`. One way to work around that is to use winpty.

So if you edit `~/.bashrc` and add this: 
```
alias roo="winpty roo"
```

It should work.

