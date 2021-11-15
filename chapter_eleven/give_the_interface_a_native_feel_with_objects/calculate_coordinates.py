from rust_package import calculate_coordinates


class FlyWeight(type):
    _instances = {}

    def __call__(cls, *args, **kwargs):
        key = str(args[0]) + str(args[1])
        if key not in cls._instances:
            cls._instances[key] = super(
                FlyWeight, cls).__call__(*args, **kwargs)
        return cls._instances[key]


class Particle(metaclass=FlyWeight):
    def __init__(self, v_x, v_y):
        self.co_dict = calculate_coordinates(v_x, v_y)

    def get_position(self, time) -> tuple:
        return self.co_dict[time]

    @property
    def times(self):
        return list(self.co_dict.keys())
