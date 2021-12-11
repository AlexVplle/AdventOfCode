inputArray = [x for x in open("input/inputDay3")]
gamma, epsilon = "", ""
for i in range(len(inputArray[0]) - 1):
    if [int(data[i]) for data in inputArray].count(0) > len(inputArray) / 2:
        gamma += "0"
        epsilon += "1"
    else :
        gamma += "1"
        epsilon += "0"
print(int(gamma, 2) * int(epsilon, 2))