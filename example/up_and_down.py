import tello_sdk
from time import sleep
#create a new drone class (this is what we will send commands to)
print("creating drone")
drone = tello_sdk.Tello()
#connect to the drone (assuming you allready have the wifi connected)
print("connecting")
drone.connect()
print("connected")
#take off
print("up")
drone.take_off()
#wait 3 seconds
sleep(3)
#land
print("down")
drone.land()