REM Pub public ipv4 address to topic
ipconfig|grep -m 1 IPv4|sed "s/IPv4 Address. . . . . . . . . . . : //g"|sed 's/   //g'| xargs -t -I {} rost.cmd status/%DENA%/IP {}
