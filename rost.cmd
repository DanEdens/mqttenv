@echo off
set _tail=%*
call set _tail=%%_tail:*%1 =%%
echo Trying: %awsip%:%awsport%
mosquitto_pub -r -h %awsip% -p %awsport% -i %DENA% -t "%1" -m "%_tail%" && echo Topic: %1 && echo Msg: %_tail%
