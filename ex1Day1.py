inputArray = [int(x) for x in open("input/inputDay1")]
numOfIncrease = 0
for i in range(1, len(inputArray)):
    if inputArray[i] > inputArray[i-1]:
        numOfIncrease += 1
print(numOfIncrease)