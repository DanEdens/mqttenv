@echo off
@set "name=%1"
@set "output=%2"
@for %%a in ("%name: ="=:="%") do @set "last=%%~a"
@FOR /F "tokens=* delims=" %%g IN ('@mosquitto_sub -h %awsip% -p %awsport% -t %1 -C 1') do (SET get=%%g)
@set last=%get%
@echo %output% = %last%
@set %output%=%last%
