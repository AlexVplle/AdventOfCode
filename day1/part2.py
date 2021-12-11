numberInFile = [int(x) for x in open("input/inputDay1")]
numOfIncrease = 0
for i in range(1, len(numberInFile) - 2):
    if numberInFile[i + 2] > numberInFile[i - 1]:
        numOfIncrease += 1
print(numOfIncrease)