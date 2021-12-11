depth, forward = 0, 0
for i in [x.split(" ") for x in open("input/inputDay2")]:
    number = int(i[1])
    if i[0] == "forward":
        forward += number
    elif i[0] == "up":
        depth -= number
    else:
        depth += number
print(depth * forward)
