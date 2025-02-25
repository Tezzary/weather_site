
ssh 192.168.0.48 '
    killall firefox

    

    export DISPLAY=:0
    nohup firefox --kiosk "vceatarcalculator.com" > /dev/null 2>&1 & 
    disown
    exit
'