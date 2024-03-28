# Toggles - Toggle Binds for Sway

Toggles is a simple program to create toggle binds for arbitrary applications
by `app_id` / `class` (for XWayland applications).

## Usage
The program takes an `app_id` (or windows class) as a matching criterion and a `command`.

Toggles uses the `app_id` to check via sway-IPC if a window with that ID is in view.

If not the window is either moved to the current workspace or launched via the
launch command.

## Installation
```sh
git clone https://github.com/feschber/toggles
cd toggles
cargo build --release
sudo mkdir -p /usr/local/bin/
sudo mv target/release/toggles /usr/local/bin/
```

## Additional Arguments

```man
Usage: toggles [OPTIONS] <APP_ID> <COMMAND>

Arguments:
  <APP_ID>   app_id or window class for xwayland windows to toggle
  <COMMAND>  command to spawn the application

Options:
  -m, --mark <MARK>  optional matching criteria mark
  -f, --floating     whether or not the window should be floating
  -h, --help         Print help
  -V, --version      Print version
```

## Example Sway Config

```sh
# a toggle terminal that attaches a tmux session
bindsym $mod+Return exec toggles "foot"                           "foot bash -c 'tmux new -As home'"

# tiled toggles
bindsym Mod1+f      exec toggles "org.mozilla.firefox"            "flatpak run org.mozilla.firefox"
bindsym Mod1+t      exec toggles "org.mozilla.Thunderbird"        "flatpak run org.mozilla.Thunderbird"
bindsym Mod1+d      exec toggles "discord"                        "flatpak run com.discordapp.Discord"

# floating toggles
bindsym Mod1+s      exec toggles --floating "Signal"              "flatpak run org.signal.Signal"
bindsym Mod1+m      exec toggles --floating "Spotify"             "flatpak run com.spotify.Client"
bindsym Mod1+n      exec toggles --floating "org.gnome.Nautilus"  "nautilus"
```
