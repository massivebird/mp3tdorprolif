# MP3 TDOR Prolif

Fixes MP3 files with missing release date metadata.

> [!CAUTION]
> This program OVERWRITES MP3 FILES!! Please back up your music library before proceeding!

🦀 Written in Rust

## Why and how

I was using [THIS-IS-NOT-A-BACKUP/zspotify](https://github.com/THIS-IS-NOT-A-BACKUP/zspotify) for a long time before I realized that the MP3s it produced were missing certain metadata fields. While the MP3s contained their release dates in some form, certain softwares could not find said dates.

This program takes the date that _does_ exist — presumably under the `TDOR` tag — and copies the data to other tags.
