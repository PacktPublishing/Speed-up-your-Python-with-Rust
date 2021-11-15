from .calculate_coordinates import Particle


test = Particle(4, 6)
test_two = Particle(3, 8)
test_three = Particle(4, 6)

print(id(test))
print(id(test_three))
print(id(test_two))
