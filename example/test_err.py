import tello_sdk
s = tello_sdk.State()
try:
	s.err()
except tello_sdk.TelloErr as e:
	print("caught")