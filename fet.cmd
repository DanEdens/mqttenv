REM File get
@REM @echo off
set "name=%1"
for %%a in ("%name:/=" "%") do set "last=%%~a"
mosquitto_sub -h %awsip% -p %awsport% -t commands/%1 -C 1 >> %last%.cmd
