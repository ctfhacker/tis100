# TIS100 Emulator

TIS100 emulator built for the terminal as a fun Rust project. Effort streamed
[on Twitch](https://twitch.tv/corydu). No guarentees of matching the game, mostly just
building a fun project for a reason to stream some Rust development.

```
+----------------------+      +----------------------+
| ACC:    0 BAK:    0  |      | ACC:    0 BAK:    0  |
+----------------------+      +----------------------+
|> ADD 1               |      |> ADD 2               |
|  SAV                 |      |  SUB 400             |
|  ADD 1               |      |                      |
|  SWP                 |      |                      |
|  NEG                 |      |                      |
|                      |      |                      |
+----------------------+      +----------------------+

+----------------------+      +----------------------+
| ACC:    0 BAK:    0  |      | ACC:    0 BAK:    0  |
+----------------------+      +----------------------+
|> ADD -400            |      |> ADD 1               |
|                      |      |  SUB 2               |
|                      |      |  SUB 4               |
|                      |      |  SUB 5               |
|                      |      |                      |
|                      |      |                      |
+----------------------+      +----------------------+
```

# Reference

* [Reference manual](https://www.zachtronics.com/images/TIS-100P%20Reference%20Manual.pdf)
