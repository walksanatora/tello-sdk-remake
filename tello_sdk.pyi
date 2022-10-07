class State:
	pass

class Flip:
	pass

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

	def disconnect(self) -> str:
		"""
		disconnects from the drone
		"""

	def send_command(self,command: str, acked: bool) -> str:
		"""
		sends a command directly to the drone
		do not use unless you read the SDK docs
		"""
	
	def __repr__(self):
		"""
		python formatting!
		"""

	def take_off(self):
		"""
		performs the automated take off
		"""
	
	def land(self):
		"""
		tells the drone to land safely
		"""

	def emergency(self):
		"""
		tells the drone to land unsafely
		(propellers immedietly off)
		"""
	
	def up(self,distace: int):
		"""
		moves the drone up `distance`cm
		"""
	def down(self,distace: int):
		"""
		moves the drone down `distance`cm
		"""
	def left(self,distace: int):
		"""
		moves the drone left `distance`cm
		"""
	def right(self,distace: int):
		"""
		moves the drone right `distance`cm
		"""
	def forward(self,distace: int):
		"""
		moves the drone forward `distance`cm
		"""
	def back(self,distace: int):
		"""
		moves the drone back `distance`cm
		"""

	def cw(self,mili_degs: int):
		"""
		rotates the drone a specified ammount of milidegres (0.001) clock wise
		"""
	def ccw(self,mili_degs: int):
		"""
		rotates the drone a specified ammount of milidegres (0.001) counter clock wise
		"""
	
	def flip(self, direction: Flip):
		"""
		flips the drone a specified direction
		"""
	
	def go(self,x:int,y:int,z:int,speed:int):
		"""
		moves the drone relative, x,y,z should be in the range {-1000,1000}
		also has a speed parameter
		"""

	def curve(self,x1: int,y1: int,z1: int,x2: int,y2: int,z2: int,speed:int):
		"""
		curves the drone from point 1 to point 2, numbers in range {-1000,1000}
		also has a speed parameter
		"""

	def speed(self, speed: int):
		"""
		sets the speed of the drone
		"""