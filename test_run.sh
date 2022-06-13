#!/usr/bin/env bash
Xephyr -screen 1200x700 :5 & sleep 1 ; DISPLAY=:5 cargo run
