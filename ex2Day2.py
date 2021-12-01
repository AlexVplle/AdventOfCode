numberInFile = [int(x) for x in open("input/inputDay1")]
numOfIncrease = 0
for i in range(len(numberInFile) - 2):
    if sum([numberInFile[i+x] for x in range(0,3)]) > sum([numberInFile[i+x] for x in range(-1,2)]):
        numOfIncrease += 1
print(numOfIncrease)
