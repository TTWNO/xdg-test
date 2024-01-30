# `xdg-test`: Test GlobalShortcut portal

- Read the [XDG docs](https://flatpak.github.io/xdg-desktop-portal/docs/requests.html)

## Testing

### KDE Portals (no-dev)

- This only works with the existing implementation of the shortcuts portal, not with dev versions.
    - Also, KDE has some bug that actually makes this application crash.
- Install `plasma-meta xdg-desktop-protal xdg-desktop-portal-kde`
- Run `systemctl start --user plasma-xdg-desktop-portal-kde`
- Alternate tester:
    - Make sure you have some free space, since the installation of https://invent.kde.org/libraries/xdg-portal-test-kde will not work if you have less than 3% space. Doesn't matter how much that is.... Sigh!
- To start plasma, run `startplasma-wayland`
- To start the test app, follow the instructions from the link above.
- To stop plasma: `qdbus org.kde.ksmserver /KSMServer logout 0 0 0`

### GNOME Portals (dev)

- TODO
