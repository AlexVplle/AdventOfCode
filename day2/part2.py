inputArray = [x.split(" ") for x in open("input/inputDay2Part2")]
depth, forward, aim = 0, 0, 0
for i in inputArray:
    if i[0] == "forward":
        forward += int(i[1])
        depth += aim * int(i[1])
    elif i[0] == "up":
        aim -= int(i[1])
    else:
        aim += int(i[1])
print(depth * forward)