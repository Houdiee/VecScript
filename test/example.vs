u : num = sin(50 deg) * cos(25)
pie : num = (pi * 2)/2

m : mtrx = [[1, 3, 4], [2, 14, 5], [13, 3, 5]]
rows = m.rows
cols = m.cols

v1 : vec = <3, 4>

v2 = <3, 4> # implicitly has vec type

dot_prod : num = dot(v1, v2)
dot_prod2 = dot(v1, v2)
cross_prod : vec = cross(v1, v2)

v1_scaled = v1 * 3

solve x in 5 + x = 3

x2 = 32 + x

mag = |v1|
my_normalized = v1.normalize

let u = 42 in
y = x^u

# cant access u here
