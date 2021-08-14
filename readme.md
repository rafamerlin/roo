Roo (🦘) means Reach Out and Open

This will find folders or files based on the search argument use in the settings.
Then after this is found it will call the command we want with it or its folder as argument.

Let's say you have this config in your .roo file:


```
walkdir: fd
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

If you run `roo vs` or `roo vs .`
It will search for all the files on that path that have the extension `.sln` and if there's more than one it will ask you which one you want opened, then it will open it on visual studio.