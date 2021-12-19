import sys
import numpy as np

def get_all_rotations():
    elements = [-1, 0, 1]
    matrices = [np.array([a, b, c, d, e, f, g, h, i]).reshape(3,3)
                for a in elements
                for b in elements
                for c in elements
                for d in elements
                for e in elements
                for f in elements
                for g in elements
                for h in elements
                for i in elements]

    matrices = list(filter(
        lambda mat: np.linalg.det(mat) == 1 and
        np.array_equal(mat.dot(mat.T), np.eye(3)),
        matrices))

    return matrices


rotations = get_all_rotations()

def get_input():
    with open(sys.argv[1]) as f:
        rawlines = f.readlines()

        current_key = -1
        data = []
        for line in rawlines:
            if line.startswith('---'):
                current_key += 1
                data.append([])
                continue

            if line.strip():
                data[current_key].append(line.split(','))

    return [np.array(coords, dtype=int) for coords in data]


def find_best_transform(pointsA, pointsB):
    best_rotation = np.eye(3)
    best_translation = np.zeros(3)
    max_coincidences = 0
    for rot in rotations:
        pointsB_rot = rot.dot(pointsB.T).T
        trans = np.array(
                [pointsA - point for point in pointsB_rot]).reshape(-1, 3)
        values, counts = np.unique(trans, return_counts=True, axis=0)
        coincidences = np.max(counts)
        if coincidences > max_coincidences:
            max_coincidences = coincidences
            best_rotation = rot
            best_translation = values[np.argmax(counts)]

    return max_coincidences, best_rotation, best_translation


if __name__ == '__main__':
    data = get_input()
    global_map = data.pop(0)

    beacon_locations = []
    while len(data) > 0:
        print("sets remaining: {}".format(len(data)))
        transforms = [(idx, find_best_transform(global_map, point_set))
                      for idx, point_set in enumerate(data)]

        best = max(transforms, key=lambda t: t[1][0])
        idx, (_, rot, trans) = best
        points = data.pop(idx)
        points_rt = rot.dot(points.T).T + trans
        global_map = np.unique(
                        np.concatenate(
                            (global_map, points_rt),
                            axis=0),
                        axis=0)

        beacon_locations.append(trans)

    print("Number of scanners: {}".format(len(global_map)))

    beacon_locations = np.array(beacon_locations).reshape(-1, 3)
    beacon_distances = np.array(
            [np.abs(beacon_locations - beacon)
             for beacon in beacon_locations]).reshape(-1, 3)

    max_distance = np.max(np.sum(beacon_distances, axis=1))
    print("Max beacon manhatten distance: {}".format(max_distance))


