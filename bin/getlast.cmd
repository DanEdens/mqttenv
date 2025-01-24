REM i forgot why this is seperate. no log? 
@echo off
set "name=%1"
@REM for %%a in ("%name: ="=:="%") do set "last=%%~a"
FOR /F "tokens=* delims=" %%g IN ('mosquitto_sub -h %awsip% -p %awsport% -t %1 -C 1') do (SET get=%%g)
@REM set name=%name:/=_%
set last=%get%
@rem echo %name% = %last%
set %name%=%last%