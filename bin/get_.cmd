REM silent get
@echo off
set "name=%1"
for %%a in ("%name: ="=:="%") do set "last=%%~a"
FOR /F "tokens=* delims=" %%g IN ('mosquitto_sub -h %awsip% -p %awsport% -t %1 -C 1') do (SET get=%%g)
set name=%name:/=_%
set last=%get%
echo %name% = %last%
set %name%=%last%