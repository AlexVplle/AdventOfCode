depth, forward, aim = 0, 0, 0
for i in [x.split(" ") for x in open("input/inputDay2Part2")]:
    number = int(i[1])
    if i[0] == "forward":
        forward += number
        depth += aim * number
    elif i[0] == "up":
        aim -= number
    else:
        aim += number
print(depth * forward)