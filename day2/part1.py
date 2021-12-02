inputArray = [x.split(" ") for x in open("input/inputDay2")]
depth, forward = 0, 0
for i in inputArray:
    if i[0] == "forward":
        forward += int(i[1])
    elif i[0] == "up":
        depth -= int(i[1])
    else:
        depth += int(i[1])
print(depth * forward)
