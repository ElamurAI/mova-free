# Security

Please do not report security problems in public issues. Write to **ai@elamur.ai** with what you found, how to reproduce it, and the release or commit. We answer within a few days.

What counts here: anything that lets input text, a model file or a corpus file run code, read or write outside the data directory (`$MOVA_DATA`) or the sandbox of the `vfs`/`wasi-host` crates, or leak secrets. `install.sh` checks every download against its SHA-256; a way around that check is in scope too.

Only the latest release is supported.
