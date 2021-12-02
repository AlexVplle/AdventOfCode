numberInFile = [int(x) for x in open("input/inputDay1")]
numOfIncrease = 0
for i in range(1, len(numberInFile) - 2):
    if sum(numberInFile[i : i + 3]) > sum(numberInFile[i - 1 : i + 2]):
        numOfIncrease += 1
print(numOfIncrease)