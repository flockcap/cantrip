# Reproducible demo: a recorded agent response, the real TUI, real conversions.
Output assets/demo.gif
Output target/demo.mp4
Require cwebp
Require python3
Set Shell "bash"
Set FontFamily "JetBrainsMono Nerd Font"
Set FontSize 22
Set Width 1400
Set Height 820
Set Padding 36
Set Margin 16
Set MarginFill "#1d2021"
Set BorderRadius 10
Set WindowBar Rings
Set WindowBarSize 32
Set Framerate 24
Set TypingSpeed 36ms
Set Theme { "name": "cantrip", "background": "#1d2021", "foreground": "#ebdbb2", "black": "#282828", "red": "#fb4934", "green": "#b8bb26", "yellow": "#fabd2f", "blue": "#83a598", "magenta": "#d3869b", "cyan": "#8ec07c", "white": "#ebdbb2", "brightBlack": "#665c54", "brightRed": "#fb4934", "brightGreen": "#b8bb26", "brightYellow": "#fabd2f", "brightBlue": "#83a598", "brightMagenta": "#d3869b", "brightCyan": "#8ec07c", "brightWhite": "#fbf1c7", "cursor": "#fe8019", "selection": "#504945" }
Hide
Type "source tapes/setup.sh"
Enter
Sleep 2s
Show
Sleep 600ms
Type 'cantrip "convert every png here to 800px webp"'
Enter
Sleep 3s
Enter
Sleep 1s
Type@18ms "ls *.webp"
Enter
Sleep 1800ms
