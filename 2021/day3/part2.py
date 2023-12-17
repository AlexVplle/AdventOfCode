inputArray = [x for x in open("input/inputDay3")]
oxygen, co2 = inputArray, inputArray
for i in range(len(inputArray[0]) - 1):
    if [int(data[i]) for data in oxygen].count(0) > len(oxygen) / 2:
        oxygen = [data for data in oxygen if data[i] == "0"]
    else :
        oxygen = [data for data in oxygen if data[i] == "1"]
    if len(co2) > 1 :
        if [int(data[i]) for data in co2].count(0) > len(co2) / 2 :
            co2 = [data for data in co2 if data[i] == "1"]
        else : 
            co2 = [data for data in co2 if data[i] == "0"]
print(int(oxygen[0], 2) * int(co2[0], 2))


