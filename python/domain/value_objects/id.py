import uuid

class ID:
    def __init__(self):
        self._value = str(uuid.uuid4())
    
    @property
    def value(self):
        return self._value