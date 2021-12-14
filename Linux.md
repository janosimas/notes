# Table of contents

- [Table of contents](#table-of-contents)
- [General improvement](#general-improvement)
  - [Shell](#shell)
  - [cat](#cat)
  - [find](#find)
  - [finder (grep, history, files)](#finder-grep-history-files)
  - [Cheat sheet](#cheat-sheet)
- [Pipewire audio driver](#pipewire-audio-driver)
- [Find folders containing some file](#find-folders-containing-some-file)
- [Safer bash scripts](#safer-bash-scripts)

# General improvement

## Shell

- [NuShell](https://www.nushell.sh/): Shell alternative great for parsing and pipelining commands. (not bash compatible)
- zsh + [omzsh](https://ohmyz.sh/): Much improved bash

## cat

[Bat](https://github.com/sharkdp/bat): improved `cat` with syntax highlight and more

## find

[fd](https://github.com/sharkdp/fd): faster and friendlier alternative to `find`

## finder (grep, history, files)

[fzf](https://github.com/junegunn/fzf) and [skim](https://github.com/lotabout/skim): General purpose finder. Minimal differences, check comparison for the current state

## Cheat sheet

[navi](https://github.com/denisidoro/navi): Easy to find cheat sheet. Configurable and dynamic on the parameters.

# Pipewire audio driver

Better audio quality and codes.

Tested with:
- `Sony WH-1000X M3`

Run the script and reboot.

```bash
#!/bin/bash

### disable pulseaudio
systemctl --user --now disable pulseaudio.{socket,service}
systemctl --user mask pulseaudio

sudo sed -i 's/.*autospawn.*/autospawn = no/g' /etc/pulse/client.conf
sudo update-rc.d pulseaudio-enable-autospawn disable
sudo mv -v /etc/xdg/autostart/pulseaudio.desktop{,.bak}
pulseaudio --kill

systemctl --user --now enable pipewire{,-pulse}.{socket,service} pipewire-media-session.service
###

# install pipewire
sudo add-apt-repository ppa:pipewire-debian/pipewire-upstream
sudo apt install pipewire gstreamer1.0-pipewire libspa-0.2-bluetooth libspa-0.2-jack
```

# Find folders containing some file

see also: [finder alternatives](#finder-grep-history-files)

```bash
find . -type f -name 'pattern' | sed -r 's|/[^/]+$||' | sort | uniq
```
Reference: https://unix.stackexchange.com/a/111951

# Safer bash scripts

Settings for early exit on failure:
```bash
set -Eeuxo pipefail
```
Reference: https://vaneyckt.io/posts/safer_bash_scripts_with_set_euxo_pipefail/
