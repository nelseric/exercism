def saddle_points(matrix):
  potentials = set([])
  for row_index, row in enumerate(matrix):
    min_point = min(row)
    potentials.add((row_index, row.index(min_point)))

  print(matrix)
  print(potentials)

  for point in potentials:
    if matrix[point[0], point[1]] != map(lambda x: a[x][point[1]], range(len(a))).max:
      potentials.delete(point)

  return potentials
