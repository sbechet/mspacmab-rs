# mspacmab-rs

long term goal: _perfect_ hardware rust remake - a method as other to play w/ rust-lang

see mspacmab project for reverse part

# Init part

```
008d timed60Hz() --+
                   |
    +--------------+ switch test mode
    |
   \/
0000 Reset() -> 230B startupTest() -> 234B main() -> 238D execute_CORE_task()
                  ^                   ^
                  |                   |
3000 testMode() --+ (at end) ---------+ (switch mode test before & after easter egg)
```

## reset()

`goto startup_test()`

## startup_test()

```
reset hardware read input (joystick1, rack test, coin, service) = 0
hardware_sound_enable = false
hardware_flipscreen = false
hardware_start_lamp_1_player_leds = 0
hardware_start_lamp_2_player_leds = 0
hardware_coin_lockout_global = 0
hardware_coin_counter = 0
setmem(hardware_tile_ram, '@', 0x400)
setmem(hardware_color_ram, color_e.color_reset_maybe_black, 0x400)  // 0x0F
goto main()
```

## main()

```
reset hardware read input0 (joystick1, rack test, coin, service) = 0
hardware_sound_enable = false
hardware_flipscreen = false
hardware_start_lamp_1_player_leds = 0
hardware_start_lamp_2_player_leds = 0
hardware_coin_lockout_global = 0
hardware_coin_counter = 0        
reset hardware sprites
reset game state
reset task list
reset hardware read input1 (joystick2, service mode switch, player1 start button, player 2 start button, cocktail cabinet DIP)
reset hardware wave select 1, 2 and 3
reset hardware wave info 1, 2 and 3
reset hardware floating_sprite_coord_t * 8
T06_clears_color_RAM()
T00_clear_whole_screen_or_maze(0)
reset task list (outside of irq) (0xFF * 40)
loop {
    // execute_CORE_task
}
```

## testMode()

see test_mode.rs


## timed60Hz()

```
copy freqs to hardware (channel1, channel2, channel3)
for each channel :
    hard wave_t.sel config
    if channel_X_wave.num == 0:
        wave_select_1 = channel_X_effect.table[0]
```


# TODO

- [x] Tile
- [x] Sprites
- [x] DrawText
- [ ] test_mode
- [ ] Game { hardware_vars, game_vars }; impl Game { new(), run(), test_mode(), timed60Hz() }
- [ ] stm32f103 hardware using `stm32f103-vga-rs` project
