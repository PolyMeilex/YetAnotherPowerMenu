![Baner](https://i.imgur.com/WQnoAxC.png)

### Why do we need yet another power menu?

The answer is really simple... we don't,
but I really like the idea of native power menu with CSS support so here you go anyway

In its final form, it is supposed to integrate well with [YetAnotherBar](https://github.com/PolyMeilex/YetAnotherBar), but for now, it is just a standalone power menu, I'll think about integrations later

![img](https://i.imgur.com/888a0Pf.jpg)

# Installation

It will be up on AUR someday

# Example Config

```rs
Config(
    time_format: "%H:%M:%S",
    date_format: "%A, %d %B %Y",

    buttons: [
        Button(
            icon: "system-lock-screen-symbolic",
            event: Lock,
            key: "j",
        ),
        Button(
            icon: "media-playback-pause",
            event: Suspend,
            key: "k",
        ),
        Button(
            icon: "system-reboot-symbolic",
            event: Reboot,
            key: "l",
        ),
        Button(
            icon: "system-shutdown-symbolic",
            event: Shutdown,
            key: "semicolon",
        ),
        //Button(
        //    icon: "system-reboot-symbolic",
        //    event: Custom(["systemctl","reboot"]),
        //    key: "semicolon"
       	//),
    ],

)
```
