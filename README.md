Roo (🦘) means Reach Out and Open
---

0.0.8 Breaking changes: I removed the walkdir config, now you need `fd` to use this app

---

This will find folders or files based on the search argument use in the settings.
Then after this is found it will call the command we want with it or its folder as argument.

Let's say you have this config in your .roo file:


```
delay: 750
commands:
  - key: "vs"
    command: "C:\\Program Files (x86)\\Microsoft Visual Studio\\2019\\Professional\\Common7\\IDE\\devenv.exe"
    command_type: "File"
    search_by: "Extension"
    search_for: ["sln", "slnx"]
  - key: "cs"
    command: "C:\\Program Files\\Microsoft VS Code\\Code.exe"
    command_type: "Directory"
    search_by: "Extension"
    search_for: ["sln", "slnx"]
  - key: "rs"
    command: "C:\\Program Files\\Microsoft VS Code\\Code.exe"
    command_type: "Directory"
    search_by: "Extension"
    search_for: "toml",
```

Delay is the delay to open the app in ms. Default is zero, but if you use virtual desktops and want to change to another one between asking the app to open and opening it you can add a delay in there.

If you run `roo vs` or `roo vs .`
It will search for all the files on that path that have the extension `.sln` and if there's more than one it will ask you which one you want opened, then it will open it on visual studio.

### Multiple Search Values

`search_for` can be either a single value or a list of values. When using multiple values, the search will match any of them:

```yaml
commands:
  - key: "solutions"
    command: "C:\\Program Files (x86)\\Microsoft Visual Studio\\2019\\Professional\\Common7\\IDE\\devenv.exe"
    command_type: "File"
    search_by: "Extension"
    search_for: ["sln", "slnx"]  # Will find both .sln and .slnx files
```

You can also keep using a single value as before:

```yaml
commands:
  - key: "rs"
    command: "code"
    command_type: "Directory"
    search_by: "Extension"
    search_for: "toml"
```

--- 
Troubleshoot
---

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

