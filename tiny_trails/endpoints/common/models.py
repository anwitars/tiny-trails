Hours = int
""" Type alias to make some things clear. """

TRAIL_DEFAULT_LIFETIME: Hours = 24 * 3
TRAIL_MINIMUM_LIFETIME: Hours = 1
TRAIL_MAXIMUM_LIFETIME: Hours = 24 * 30

TRAIL_TOKEN_HEADER = "X-Trail-Token"

# WARN: Database scheme depends on this value, do not change it without migration!
TRAIL_TOKEN_LENGTH = 32
