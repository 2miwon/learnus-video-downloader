.PHONY: build run clean help mp4 mp3 autorun

TARGET_NAME = learnus-video-downloader
EXECUTABLE = target/release/$(TARGET_NAME)

ARGS = $(filter-out $@,$(MAKECMDGOALS))
.DEFAULT_GOAL := help

build:
	@cargo build --release

run:
	@$(EXECUTABLE) $(ARGS)

autorun: build run

mp4: run
	ffmpeg -i ./output/$(ARGS).ts -c copy ./output/$(ARGS).mp4

mp3: run
	ffmpeg -i ./output/$(ARGS).ts -q:a 0 -map a ./output/$(ARGS).mp3

clean:
	@cargo clean

help:
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "  %-20s %s\n", $$1, $$2}'


