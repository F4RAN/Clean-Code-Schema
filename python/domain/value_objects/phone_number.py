import re
class PhoneNumber:
    def __init__(self, value):
        if not isinstance(value, str):
            raise ValueError("Phone Number must be a string")
        if not re.match(r"^\d{10}$", value):\
            raise ValueError("Phone number must be exactly 10 digits")
        self._value = value
        