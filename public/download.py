import requests
from bs4 import BeautifulSoup
import sys

url = sys.argv[1]
target = sys.argv[2]

r = request.get(url)
r.encoding = r.apparent_encoding

print(r.text)

soup = BeautifulSoup(r.text, "html.parser")
table = soup.find_all("table")[1]

print(table)