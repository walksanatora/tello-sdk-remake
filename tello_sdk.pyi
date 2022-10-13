class State:
	roll: int
	pitch: int
	yaw: int
	ground_velocity_x: int
	ground_velocity_y: int
	ground_velocity_z: int
	temperature_minimum: int
	temperature_maximum: int
	tof_value: int
	height: int
	battery_percentage: int
	barometer_height: float
	time: int
	ground_acceleration_x: int
	ground_acceleration_y: int
	ground_acceleration_z: int

class Flip:
	Left: Flip
	Right: Flip
	Forward: Flip
	Backward: Flip

class Tello:
	state: State
	running: bool
	ack_ms: int
	def __init__(self):
		"""
		create a new drone class, this will automatically create the UDP sockets
    	errors if the sockets cannot be unblocked
		"""
	
	def connect(self) -> str:
		"""
		connects to the drone
        also setups the async threads for allowing values to change
        errors if the drone does not respond in a set ammount of time
        or some other error (usally a socket read error)
		"""

	def disconnect(self):
		"""
		disconnects from the drone
		"""

	def send_command(self,command: str, acked: bool) -> str:
		"""
		sends a command directly to the drone
		do not use unless you read the SDK docs
		"""
	
	def __repr__(self) -> str:
		"""
		python formatting!
		"""

	def take_off(self) -> str:
		"""
		performs the automated take off
		"""
	
	def land(self) -> str:
		"""
		tells the drone to land safely
		"""

	def emergency(self) -> str:
		"""
		tells the drone to land unsafely
		(propellers immedietly off)
		"""
	
	def up(self,distace: int) -> str:
		"""
		moves the drone up `distance`cm
		"""
	def down(self,distace: int) -> str:
		"""
		moves the drone down `distance`cm
		"""
	def left(self,distace: int) -> str:
		"""
		moves the drone left `distance`cm
		"""
	def right(self,distace: int) -> str:
		"""
		moves the drone right `distance`cm
		"""
	def forward(self,distace: int) -> str:
		"""
		moves the drone forward `distance`cm
		"""
	def back(self,distace: int) -> str:
		"""
		moves the drone back `distance`cm
		"""

	def cw(self,mili_degs: int) -> str:
		"""
		rotates the drone a specified ammount of milidegres (0.001) clock wise
		"""
	def ccw(self,mili_degs: int) -> str:
		"""
		rotates the drone a specified ammount of milidegres (0.001) counter clock wise
		"""
	
	def flip(self, direction: Flip) -> str:
		"""
		flips the drone a specified direction
		"""
	
	def go(self,x:int,y:int,z:int,speed:int) -> str:
		"""
		moves the drone relative, x,y,z should be in the range {-1000,1000}
		also has a speed parameter
		"""

	def curve(self,x1: int,y1: int,z1: int,x2: int,y2: int,z2: int,speed:int) -> str:
		"""
		curves the drone from point 1 to point 2, numbers in range {-1000,1000}
		also has a speed parameter
		"""

	def speed(self, speed: int) -> str:
		"""
		sets the speed of the drone
		"""
	
	def ext_led_color(self,r: int,g: int,b: int) -> str:
		"""
		sets the color of the top led
		"""

	def ext_led_pulse(self,r: int,g: int,b: int,rate: float) -> str:
		"""
		pulses the top led on and off at the specific rate
		"""	
	def ext_led_blink(self,r1: int,g1: int,b1: int,r2: int,g2: int,b2: int,rate: float) -> str:
		"""
		swaps between two colors at a speciied rate
		"""
	def ext_matrix_display(self,colors: str) -> str:
		"""
		draw colors directly on the matrix display
		"""
	def ext_matrix_char(self,color: str,string: str,rate: float) -> str:
		"""
		display a single charachter on the matrix (or "heart")
		"""
	def ext_matrix_brightness(self,brightness: int) -> str:
		"""
		sets the brightness of the LED display
		"""
	def ext_matrix_scroll(self,dir: str,color: str, string: str, rate: float) -> str:
		"""
		scrolls text across the LED display
		"""