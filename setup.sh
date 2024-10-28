#!/bin/sh

dotnet new console -o /tmp/csharp_demo --use-program-main
cargo new /tmp/rust_demo

zellij action new-tab --layout layout_dotnet.kdl
zellij action new-tab --layout layout_rust.kdl

