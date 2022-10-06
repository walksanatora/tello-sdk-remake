import tello_sdk
from time import sleep
#create a new drone class (this is what we will send commands to)
drone = tello_sdk.Tello()
#connect to the drone (assuming you allready have the wifi connected)
drone.connect()
#take off
drone.take_off()
#wait 3 seconds
sleep(3)
#land
drone.land()