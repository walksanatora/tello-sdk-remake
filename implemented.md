# Flip
Flip has 4 values<br>
`Flip.Left`,`Flip.Right`,`Flip.Forward`,`Flip.Backward`<br>
these are to be used for in the `flip` command for your drone Ex:
```py
drone = Tello()
drone.connect()
drone.take_off()
drone.flip(Flip.Backward)
drone.land()
```

# Tello
Tello is the base class<br>
you should first initiliaze it then connect<br>
```py
drone = Tello()
drone.connect()
```
<br>
after you connect you can start to send commands to it, a list of all implemented functions are below<br>
<br>
you notice how they all have `self` you do not need to provide that, it is provided automatically<br>
<br>
connect(self) 	-- connects to the drone<br>
take_off(self)  -- tell the drone to get off the ground and turn on it's propellers<br>
land(self)		-- tells the drone to land *safely*<br>
emergency(self) -- tells the drone to cut power the the engines and *fall*<br>
up(self,distance)	-- moves the drone up `distance`cm<br>
down(self,distance)	-- moves the drone down `distance`cm<br>
left(self,distance)	-- moves the drone left `distance`cm<br>
right(self,distance)	-- moves the drone right `distance`cm<br>
forward(self,distance)	-- moves the drone forward `distance`cm<br>
back(self,distance)	-- moves the drone back `distance`cm<br>
cw(self,rot)	-- rotates the drone `rot`mdeg (0.001 degres) clockwise<br>
ccw(self,rot)	-- rotates the drone `rot`mdeg (0.001 degres) counter clockwise<br>
flip(self,dir)	-- flips the drone `dir`, to get a valid direction use the `Flip` class<br>
go(self,x,y,z,speed)	-- moves relative x,y,z at the `speed` specified<br>
curve(self,x1,y1,z1,x2,y2,z2,speed)	-- all points are a distance in cm, it curves from (x1,y1,z1) to (x2,y2,z2)<br>
rc(self,left_right,forward_back,up_down,yaw) -- simulate a controller input<br>
<br>

there are also `ext_` commands which are used when you have the Robot Master connected (the LED matrix)<br>
(they error if there is no extension)
ext_led_color(self,r,g,b) -- r,g,b should be in range 0-255<br>
ext_led_pulse(self,r,g,b,rate) -- the RGB color and the rate (0.1-2.5)Hz<br>
ext_led_blink(self,r1,g1,b1,r2,b2,g2,rate) -- pulse between RGB1 and RGB2 at (0.1-10)Hz<br>
ext_matrix_display(self,colors) -- displays colors, colors is a string of colors<br>
ext_matrix_scroll(self,dir,color,string,rate) -- scrolls a string across the matrix display (0.1-2.5)Hz<br>
ext_matrix_char(self,color,char) -- displays a single charachter, char is either a letter or "heart"<br>
ext_matrix_brightness(self,brightness) -- sets the display brightness (0-255)<br>
r,g,b are all in range 0-255<br>
color is any of
|Charachter|Color |
|----------|------|
|r         |Red   |
|b         |Blue  |
|p         |Purple|
|0         |Clear |
dir is any of
|Charachter|Direction|
|----------|---------|
|l         |Left     |
|r         |Right    |
|u         |Up       |
|d         |Down     |

Hz is Times per second so 1Hz is 1 time per second

send_command(self,command,acked) -- run the tello command `command` and if it has to be acknowledged, ***SHOULD NOT BE USED UNLESS YOU READ THE SDK USER GUIDE***<br>

the initiliased Tello class also has a few attributes
`Tello.state` a [state](#state) class containing a ton of data about the drone
`Tello.running` a boolean of whether the drone is active or not

# State
state is usually gotten from `Tello.state`<br>
and contains alot of information about the drone, here are all the values it provides<br>
they can all be accesed via State.\<value\><br>
<br>
roll					the yaw of the drone (deg)<br>
pitch					the yaw of the drone (deg)<br>
yaw						the yaw of the drone (deg)<br>
ground_velocity_x		the ground velocity in the X direction (dm/s) (decimeters/second)<br>
ground_velocity_y		the ground velocity in the Y direction (dm/s) (decimeters/second)<br>
ground_velocity_z		the ground velocity in the Z direction (dm/s) (decimeters/second)<br>
temperature_minimum		the min temperature of the main board (celcius)<br>
temperature_maximum		the max temperature of the main board (celcius)<br>
tof_value				the "Time of Flight" distance sensor<br>
height					the height relitage to take off point (cm)<br>
battery_percentage		the percent remaing of the battery (%)<br>
barometer_height		the height detected by the barometer (m)<br>
time					the time the motors have been active (s)<br>
ground_acceleration_x	the acceleration in the X direction (cm/s^2)<br>
ground_acceleration_y	the acceleration in the Y direction (cm/s^2)<br>
ground_acceleration_z	the acceleration in the Z direction (cm/s^2)<br>