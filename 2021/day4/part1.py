import re

file = open('input/inputDay4.txt', "r")

#numberRow = file.readline().strip().split(",")

tab = [re.split('\s+', x)[:5] for x in open("input/inputDay4.txt") if x != '\n'][2:]
print(tab)
tabAll = []
for i in range(0,len(tab), 5):
    print(i)
    tabAll.append(tab[i:i+5])