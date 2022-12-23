# name=$1
# output=$2
# for %%a in ("%name: ="=:="%") do @set "last=%%~a"

# FOR /F "tokens=* delims=" %%g IN ('@mosquitto_sub -h $awsip% -p %awsport% -t %1 -C 1') do (SET get=%%g)

export $2=$(mosquitto_sub -h 3.134.3.199 -p 3001 -t $1 -C 1)
# set %output%=%last%
# echo %output% = %last%
