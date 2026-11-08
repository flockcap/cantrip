# cantrip widget fish | source — Alt-G converts the prompt buffer to a command.
# Requires python3. The result stays editable; never executes.
function __cantrip_widget
    set -l result (cantrip --json -- (commandline) | python3 -c 'import json,sys; s=json.load(sys.stdin); assert not s.get("raw"); print(s["command"],end="")')
    if test $pipestatus[1] -eq 0; and test $pipestatus[2] -eq 0
        commandline --replace -- $result
    end
    commandline -f repaint
end
bind \eg __cantrip_widget
