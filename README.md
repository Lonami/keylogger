# keylogger

A pure-Rust keylogger for Windows, attaching to the raw keyboard events. No dependencies other
than `user32.lib` being available on the system, which it probably is if you can compile Rust.

This code is likely to be a good base to also log mouse or other HID events, and even simulate
them or replay them. The goal of the project is to investigate the raw events produced.

The code follows the guide [Minimal Key Logger Using RAWINPUT][1] written by Mike G. P.Mee,
where the implementation itself is pretty much borrowed from Microsoft's [Using Raw Input][2].

Another interesting project is [aquatone's ProjectEagleWatch][3], written in C, which seems
to be more wary about hiding itself, though I have not tried it.

As with most Rust projects, this one is also dual licensed under either of

* Apache License, Version 2.0 ([LICENSE-APACHE] or
  http://www.apache.org/licenses/LICENSE-2.0)

* MIT license ([LICENSE-MIT] or http://opensource.org/licenses/MIT)

at your option.

[1]: https://www.codeproject.com/articles/297312/minimal-key-logger-using-rawinput
[2]: https://docs.microsoft.com/en-us/windows/win32/inputdev/using-raw-input
[3]: https://github.com/aquatone/ProjectEagleWatch
[LICENSE-APACHE]: LICENSE-APACHE
[LICENSE-MIT]: LICENSE-MIT
