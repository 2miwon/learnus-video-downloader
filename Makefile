.PHONY: build run clean help mp4 mp3 autorun

TARGET_NAME = learnus-video-downloader
EXECUTABLE = target/release/$(TARGET_NAME)

ARGS := $(wordlist 2, $(words $(MAKECMDGOALS)), $(MAKECMDGOALS))
.DEFAULT_GOAL := help

ID := $(word 1, $(ARGS))
FNAME := $(word 2, $(ARGS))

build:
	@cargo build --release

run:
	@$(EXECUTABLE) $(ID) $(FNAME)

autorun: build run

mp4: run
	ffmpeg -i ./output/$(FNAME).ts -c copy ./output/$(FNAME).mp4 && rm ./output/$(FNAME).ts

mp3: run
	ffmpeg -i ./output/$(FNAME).ts -q:a 0 -map a ./output/$(FNAME).mp3 && rm ./output/$(FNAME).ts

clean:
	@cargo clean

help:
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "  %-20s %s\n", $$1, $$2}'


