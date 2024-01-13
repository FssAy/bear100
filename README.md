# CHEAT OUTDATED, READ BEFORE MAKING AN ISSUE
With every patch it is required to update the offsets. I've uninstalled the game and will never update the cheat.

If you want to achieve the same effect - download the Frosty Editor tool, find the Bear collectable entity and change it's value to any amount of cash you want. Visit NFS modding Discord servers to see how to use the Frosty Editor and mod the game.

If you want to contribute - do a pointer scan for the Bear collectible value, find offsets that are as similar to the ones in the config file (there always will be thousands of different offsets, but these for some reason are the most reliable) and do a PR with new offsets in the [config builder](https://github.com/FssAy/bear100/blob/68ae75738b581ca24c8f5e269c38999aae029b83/bear100/config.rs#L105).

# 

### ---> !!! Important Notice !!! <---
> **Warning**
> If you are an idiot and don't know how to use the command line on Windows, just use the "NOOB EDITION". It's the same cheat but packed into a trainer UI, so you would have to be special in order to fuck this up.
>  If for some reason you don't have enough IQ to navigate to the releases page, here is a [link](https://github.com/DmitrijVC/bear100/releases/download/v1.0.3-trainer/unbound-trainer-vol3.exe) that will automatically download the "NOOB EDITION".

<img src="https://media3.giphy.com/media/v1.Y2lkPTc5MGI3NjExZXUzNjVrZmdudTRyaTE4ZnNtOHo0Zm9sbGNuMzNyNmQ0NDhpbmZxdCZlcD12MV9pbnRlcm5hbF9naWZfYnlfaWQmY3Q9Zw/x0GeFXErpcRk4/giphy.gif" style="height: 275px; left: 0px; top: 0px; opacity: 0;">

> **Warning**
> If you are an idiot and don't know how to use the command line on Windows, just use the "NOOB EDITION". It's the same cheat but packed into a trainer UI, so you would have to be special in order to fuck this up.
>  If for some reason you don't have enough IQ to navigate to the releases page, here is a [link](https://github.com/DmitrijVC/bear100/releases/download/v1.0.3-trainer/unbound-trainer-vol3.exe) that will automatically download the "NOOB EDITION".
### ---> !!! Important Notice !!! <---

<br>
<br>

# Bear100
A simple NFS Unbound cheat to change the "Bear Champ" reward.

Pretty easy way to make money on the multiplayer. <br>
The user's money is server sided, but the rewards from collectibles not, which allows this script to work.

You shouldn't be banned by using it. <br>
EA didn't implement any anti-cheat system, even on the server side. That's why when your bank money 
increases by millions of dollars for no reason nothing bad happens. <br>
Either way I am not responsible for any unwanted behaviour caused by this software, use it on your own risk.

### NOW AVAILABLE FOR VOL.03
<img src="https://i.imgur.com/znWqmWB.png" width="120">

## Usage
*If this instruction is not followed an error will occur.* <br>
Run the executable after loading the save on singleplayer, or after finding a session on multiplayer. <br>
To be 100% sure it works, just open your map and then run the executable.

Errors might contain the System Error Code. 
To find out what does the value mean visit [this](https://learn.microsoft.com/en-us/windows/win32/debug/system-error-codes--0-499-) website.

The executable does not need to be run as an administrator, but if any errors occur, it could help in some cases.

## Features
- [x] Supports both singleplayer and multiplayer
- [x] Allows to determine exact amount of the reward
- [x] Allows to easily change the offsets without recompilation
- [ ] Resets already collected bears *(might not be possible)*

## Tested on:
- game version - `STEAM 1.0.6.53569`
- game mode - `singleplayer` `multiplayer`
- day - `25 June 2023`
- os - `Windows 10 Pro x64`

## Config
The config file `config.json` is created with default values in any case it's not valid. <br>
For example when the file is missing or it's syntax is invalid.

The default config:
```json
{
  "process_name": "NeedForSpeedUnbound.exe",
  "offsets": [
    85013392,
    8,
    960,
    32,
    100
  ],
  "replace_value": 100000000,
  "timeout_s": 4,
  "refresh_rate_ms": 700,
  "show_app_info": true
}
```
To find out what each option does see the `Config` struct in `/bear100/config.rs`.

**! WARNING!** <br>
The `replace_value` allows values up to `4.294.967.295`, but values above `2.147.483.647` should be avoided, the usage of such values was not tested and might crash your game or even corrupt the save file. <br>
`100.000.000` should be enough money for your whole gameplay.

## Compilation
To build the project run command `cargo build --bin bear100 --release`.

**! WARNING!** <br>
This software works only on 64 bit versions of Windows.

## Results
| Before                                                           | After                                                            |
|:----------------------------------------------------------------:|:----------------------------------------------------------------:|
| <img src="https://i.imgur.com/6GVuLoO.png" width=50% height=50%> | <img src="https://i.imgur.com/eAvvHZ0.png" width=50% height=50%> |

[Click for video showcase.](https://youtu.be/HlnrDJioqLQ)
