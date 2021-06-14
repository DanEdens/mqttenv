setlocal enabledelayedexpansion
FOR %%i IN (*.cmd) DO ( call :store %%i )
goto:eof

:store
set name=%1
if not '%name%'=='init' ( mklink %name% mqttenv/%name% )
goto:eof

:bak
cd ..
@rem mklink "fet.cmd" "mqttenv/fet.cmd"
mklink "fost.cmd" "mqttenv/fost.cmd"
mklink "get.cmd" "mqttenv/get.cmd"
mklink "ipr.cmd" "mqttenv/ipr.cmd"
mklink "nrost.cmd" "mqttenv/nrost.cmd"
mklink "post.cmd" "mqttenv/post.cmd"
mklink "rost.cmd" "mqttenv/rost.cmd"
mklink "store.cmd" "mqttenv/store.cmd"
@if not defined %awsip% ( set /p IP="Mqtt ip?" && setx awsip %IP% )
@if not defined %awsport% ( set /p PORT="Mqtt port?" && setx awsport %PORT%)
@if not defined %DENA% ( set /p NAME="Device name?" && setx DENA %NAME% )
