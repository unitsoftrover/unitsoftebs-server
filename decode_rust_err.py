#!/usr/bin/python

# str1 = "this is string example....wow!!!"
# str1 = str.encode('UTF-811') 
# print("Encoded String: ", str(str1))
# print("Decoded String: " + str1.decode('UTF-8','strict'))

#content = b"LINK : fatal error LNK1104: \xce\xde\xb7\xa8\xb4\xf2\xbf\xaa\xce\xc4\xbc\xfe\xa1\xb0D:\\Rust\\diesel\\target\\debug\\deps\\show_posts.exe\xa1\xb1\r\n"
#content = b" note: Non-UTF-8 output: msvcrt.lib(chkstk.obj) : fatal error LNK1112: \xc4\xa3\xbf\xe9\xbc\xc6\xcb\xe3\xbb\xfa\xc0\xe0\xd0\xcd\xa1\xb0x86\xa1\xb1\xd3\xeb\xc4\xbf\xb1\xea\xbc\xc6\xcb\xe3\xbb\xfa\xc0\xe0\xd0\xcd\xa1\xb0x64\xa1\xb1\xb3\xe5\xcd\xbb\r\n"
#content = b"Non-UTF-8 output: LINK : fatal error LNK1104: \xce\xde\xb7\xa8\xb4\xf2\xbf\xaa\xce\xc4\xbc\xfe\xa1\xb0D:\\Rust\\diesel\\target\\debug\\deps\\show_posts.exe\xa1\xb1\r\n"
#content = b"LINK : fatal error LNK1181: \xce\xde\xb7\xa8\xb4\xf2\xbf\xaa\xca\xe4\xc8\xeb\xce\xc4\xbc\xfe\xa1\xb0sqlite3.lib\xa1\xb1\r\n"
#content = b" Non-UTF-8 output: D:\\Rust\\web\\diesel\\target\\debug\\deps\\libactix_rt-f2fc370db0957ef5.rlib : fatal error LNK1127: \xbf\xe2\xd2\xd1\xcb\xf0\xbb\xb5\r\n"
content = b"error LNK2019: \xce\xde\xb7\xa8\xbd\xe2\xce\xf6\xb5\xc4\xcd\xe2\xb2\xbf\xb7\xfb\xba\xc5 sqlite3_close\xa3\xac\xba\xaf\xca\xfd _ZN88_$LT$diesel..sqlite..connection..raw..RawConnection$u20$as$u20$core..ops..drop..Drop$GT$4drop17hebadd3f87c32f0ecE \xd6\xd0\xd2\xfd\xd3\xc3\xc1\xcb\xb8\xc3\xb7\xfb\xba\xc5\r\nlibdiesel-c176703ad01b6aca.rlib(diesel-c176703ad01b6aca.w2y27beveli98uc.rcgu.o) : error LNK2019: \xce\xde\xb7\xa8\xbd\xe2\xce\xf6\xb5\xc4\xcd\xe2\xb2\xbf\xb7\xfb\xba\xc5 sqlite3_finalize\xa3\xac\xba\xaf\xca\xfd _ZN85_$LT$diesel..sqlite..connection..stmt..Statement$u20$as$u20$core..ops..drop..Drop$GT$4drop17h9330ae8413c822c0E \xd6\xd0\xd2\xfd\xd3\xc3\xc1\xcb\xb8\xc3\xb7\xfb\xba\xc5\r\nlibdiesel-c176703ad01b6aca.rlib(diesel-c176703ad01b6aca.3643qv75qvf4lkf2.rcgu.o) : error LNK2019: \xce\xde\xb7\xa8\xbd\xe2\xce\xf6\xb5\xc4\xcd\xe2\xb2\xbf\xb7\xfb\xba\xc5 sqlite3_value_text\xa3\xac\xba\xaf\xca\xfd _ZN6diesel6sqlite10connection12sqlite_value11SqliteValue9read_text17ha75dcc0916607ed2E \xd6\xd0\xd2\xfd\xd3\xc3\xc1\xcb\xb8\xc3\xb7\xfb\xba\xc5\r\nlibdiesel-c176703ad01b6aca.rlib(diesel-c176703ad01b6aca.3643qv75qvf4lkf2.rcgu.o)"
print( content.decode("gbk"))