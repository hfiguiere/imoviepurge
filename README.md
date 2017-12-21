imoviepurge
===========

A utility to list files that are in both some directories and an
iMovie Library in order to delete then to save space.

How to build
------------

You need Rust.

Just do `cargo build` to build the tool.

Running
-------

You can run it with `cargo run -- -l path/to/imovie/library -s
path/to/videos`.  It will print out all the files in `path/to/videos`
that are also found somewhere in the original medias for
`path/to/imovie/library`.

Implementation details:
It compares the files based on name and byte size.


Credit
------

Written by Hubert Figuiere <hub@figuiere.net>