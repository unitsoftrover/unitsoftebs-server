#!/usr/bin/env python3

# This script could be used for actix-web multipart example test
# just start server and run client.py

import json
import asyncio
import aiohttp
import time

async def req():
    resp = await aiohttp.ClientSession().request(
        "post", 'http://localhost:8080/company',
        data=json.dumps({"company_name": "unitsoft111", "is_headquater":True, "gen_tel": "22817058", "gen_fax":"22817058-8009", "website":"http://www.unitsoft.com.cn", "office_code":"SH", "client_type":"C"}),
        headers={"content-type": "application/json"})
    print(str(resp))
    print(await resp.text())
    assert 200 == resp.status

async def reqcompany():

    for i in range(10):    
        print("------------------" + "before loop:" + str(i) + "------------------")
        time1 = time.time()
        resp = await aiohttp.ClientSession().request(
            "get", 'http://localhost:8080/company/O0000001',
            data=json.dumps({"company_name": "unitsoft111", "is_headquater":True, "gen_tel": "22817058", "gen_fax":"22817058-8009", "website":"http://www.unitsoft.com.cn", "office_code":"SH", "client_type":"C"}),
            headers={"content-type": "application/json"})
        print(str(resp))
        print(await resp.text())
        assert 200 == resp.status


        print("------------------" + "after loop:" + str(i) + " use time:" + time.strftime("%S", time.localtime(time.time() - time1)) + "------------------")
    
#, "is_headquater":True, "gen_tel": "22817058", "gen_fax":"22817058-8009", "website":"http://www.unitsoft.com.cn", "office_code":"SH", "client_type":"C"
#      company_id      : 1,
#      company_code    : "C0000001".to_string(),
#      company_name    : "unitsoft".to_string(),
#      is_headquater   : true,
#      gen_tel         : "021-22817058".to_string(),
#      gen_fax         : "021-22817058-8009".to_string(),
#      website         : "http://www.unitsoft.com.cn".to_string()
#      client_type         :  "C".to_string() 

asyncio.get_event_loop().run_until_complete(reqcompany())
