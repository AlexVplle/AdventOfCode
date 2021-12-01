file = open("input/inputEx1", "r")
inputArray = file.readlines()
file.close()
inputArray = list(map(lambda x : int(x), inputArray))
numOfIncrease = 0
for i in range(1, len(inputArray)):
    if inputArray[i] > inputArray[i-1]:
        numOfIncrease += 1
print(numOfIncrease)