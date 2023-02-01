# Bear100
A simple NFS Unbound cheat to change the "Bear Champ" reward.

Pretty easy way to make money on the multiplayer. <br>
The user's money is server sided, but the rewards from collectibles not, which allows this script to work.

You shouldn't be banned by using it. <br>
EA didn't implement any anti-cheat system, even on the server side. That's why when your bank money 
increases by millions of dollars for no reason nothing bad happens. <br>
Either way I am not responsible for any unwanted behaviour caused by this software, use it on your own risk.

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
- game version - `STEAM 1.0.6.3374`
- game mode - `singleplayer` `multiplayer`
- day - `01 February 2023`
- os - `Windows 10 Pro x64`

## Config
The config file `config.json` is created with default values in any case it's not valid. <br>
For example when the file is missing or it's syntax is invalid.

The default config:
```json
{
  "process_name": "NeedForSpeedUnbound.exe",
  "offsets": [
    83016192,
    8,
    960,
    32,
    116
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
| Before                                                                                             | After                                        |
|----------------------------------------------------------------------------------------------------|----------------------------------------------|
| <p style="text-align:center;"><img src="https://i.imgur.com/6GVuLoO.png" width=50% height=50%></p> | <p style="text-align:center;"><img src="https://i.imgur.com/eAvvHZ0.png" width=50% height=50%></p> |

[Click for video showcase.](https://youtu.be/HlnrDJioqLQ)
