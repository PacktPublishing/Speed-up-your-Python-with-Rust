import numpy as np


class MatrixModel:

    @property
    def weights_matrix(self) -> np.array:
        return np.array([
            [3, 2],
            [1, 4]
        ])

    def calculate_times(self, distance: int,
                        traffic_grade: int) -> dict:
        inputs = np.array([
            [distance],
            [traffic_grade]
        ])
        result = np.dot(self.weights_matrix, inputs)
        return {
            "car time": result[0][0],
            "truck time": result[1][0]
        }

    def calculate_parameters(self, car_time: int,
                             truck_time: int) -> dict:
        inputs = np.array([
            [car_time],
            [truck_time]
        ])
        result = np.dot(np.linalg.inv(self.weights_matrix),
                        inputs)
        return {
            "distance": result[0][0],
            "traffic grade": result[1][0]
        }


if __name__ == "__main__":
    test = MatrixModel()
    times = test.calculate_times(distance=10, traffic_grade=3)
    print(f"here are the times: {times}")

    parameters = test.calculate_parameters(
        car_time=times["car time"], truck_time=times["truck time"]
    )
    print(f"here are the parameters: {parameters}")



