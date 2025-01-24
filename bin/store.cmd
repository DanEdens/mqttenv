@echo off
REM TODO, make aoe if no par, targeted if par
setlocal enabledelayedexpansion
FOR %%i IN (*.cmd) DO ( call :store %%i )
goto:eof

:store
set name=%1
set name=%name:~0,-4%
if not '%name%'=='store' ( call fost.cmd commands/%name% %1)
goto:eof