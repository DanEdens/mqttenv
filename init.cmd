cd ..
mklink "fet.cmd" "mqttenv/fet.cmd"
mklink "fost.cmd" "mqttenv/fost.cmd"
mklink "get.cmd" "mqttenv/get.cmd"
mklink "ipr.cmd" "mqttenv/ipr.cmd"
mklink "nrost.cmd" "mqttenv/nrost.cmd"
mklink "post.cmd" "mqttenv/post.cmd"
mklink "rost.cmd" "mqttenv/rost.cmd"
mklink "store.cmd" "mqttenv/store.cmd"
set /p IP="Mqtt ip?"
set /p PORT="Mqtt port?"
set /p NAME="Device name?"

setx awsip %IP%
setx awsport %PORT%
setx DENA %NAME%