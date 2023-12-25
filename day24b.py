from z3 import Real, sat, simplify, Solver

solver = Solver()

stone_x = Real('x')
stone_y = Real('y')
stone_z = Real('z')
stone_xv = Real('xv')
stone_yv = Real('yv')
stone_zv = Real('zv')

hail = [
    (275325627102914, 177556324137106, 279758114394131, 249, 405, -531, Real('t0')),
    (284428334220238, 231958436807561, 189800593445547, 237, 140, -111, Real('t1')),
    (208260774362545, 354915461185166, 308973039318009, 128, -159, -65, Real('t2')),
]

for (hail_x, hail_y, hail_z, hail_xv, hail_yv, hail_zv, time) in hail:
    solver.add(stone_x + stone_xv * time == hail_x + hail_xv * time)
    solver.add(stone_y + stone_yv * time == hail_y + hail_yv * time)
    solver.add(stone_z + stone_zv * time == hail_z + hail_zv * time)

if solver.check() == sat:
    model = solver.model()
    
    print("Stone X: "+str(model[stone_x]))
    print("Stone Y: "+str(model[stone_y]))
    print("Stone Z: "+str(model[stone_z]))
    print("Stone X Velocity: "+str(model[stone_xv]))
    print("Stone Y Velocity: "+str(model[stone_yv]))
    print("Stone Z Velocity: "+str(model[stone_zv]))
    
    print("Sum: "+str(simplify(model[stone_x] + model[stone_y] + model[stone_z])))
