"""Utilities for making mpi use safer and easier."""
import sys, os

class FakeCommunicator:
	def __init__(self):
		self.size = 1
		self.rank = 0

FAKE_WORLD = FakeCommunicator()

# Allow us to disable real mpi, creating only a simple placeholder object
# that will work for trivially parallelizable code run with only 1 task
if "DISABLE_MPI" in os.environ and os.environ["DISABLE_MPI"].lower() in ["true","1"]:
	COMM_WORLD = FAKE_WORLD
	disabled   = True
else:
	# We transparently pass through all the mpi4py.MPI stuff, but also add a cleanup
	# hook. On scinet I found that uncaught exceptions did not cause mpi to abort,
	# leading to thousands of wasted CPU hours. That may have been system-specific,
	# though. Perhaps this isn't necessary in general.
	from mpi4py.MPI import *
	def cleanup(type, value, traceback):
		sys.__excepthook__(type, value, traceback)
		COMM_WORLD.Abort(1)
	sys.excepthook = cleanup
	disabled = False
