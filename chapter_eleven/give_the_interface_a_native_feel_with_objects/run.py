from .calculate_coordinates import Particle


particle = Particle(20, 30)

x_positions = []
y_positions = []

for t in particle.times:
    x, y = particle.get_position(t)
    x_positions.append(x)
    y_positions.append(y)
