# 🧸 codeybot

A discord bot written in Rust using serenity.

The bot is being actively developed on https://twitch.tv/celiacode

Test the bot out by joining the Discord server!
https://discord.gg/tuetDM3D8m

## Features

- React roles: dynamically adds roles to users via emoji reactions
- Commands: several commands which respond with memes and reaction images
- Points system: interacts with Postgresql database to allow users to collect points
- Tamagotchi-like features: Codey gets hungry, can be fed and (soon) evolved!

## Current commands

### Utility commands

#### !points

Retrieves how many points you currently have in the server

#### !twitch

Brings up twitch stream

#### !test

Tests the bot

#### !vip

Assigns vip role if the user has sufficient amount of points

### Fun / meme commands

#### !surprise !panic !jam !realizing !cry !rocket !skitty

Sends corresponding reaction images

#### !ghibli movie-name

Replies with information about the requested ghibli film

### Virtual Pet Commands

#### !hunger !feed

Check hunger with !hunger and feed the bot with !feed
