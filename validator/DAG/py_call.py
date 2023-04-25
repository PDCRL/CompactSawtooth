
from ctypes import cdll
from ctypes import c_int
import time
# load the library
lib = cdll.LoadLibrary('./libgeek.so')

# create a Geek class
class Geek(object):
  
    # constructor
    def __init__(self):
  
        # attribute
        self.obj = lib.Geek_new()
  
    # define method
    def DAG_prune(self):
        lib.DAG_prune(self.obj)

    # define method
    def Smart_Validator(self):
        lib.Smart_Validator(self.obj)

    # define method
    def DAG_create(self):
        lib.DAG_create(self.obj)        

    # define method
    def DAG_create2(self):
        lib.DAG_create2(self.obj)

    # define method
    def DAG_create3(self):
        lib.DAG_create3(self.obj)

    # define method
    def DAG_select(self):
        return lib.DAG_select(self.obj)

    # define method
    def Smart_Validator(self):
        return lib.Smart_Validator(self.obj)

    # define method
    def DAG_delete(self, int):
        lib.DAG_delete(self.obj, int )
  
# create a Geek class object
dag = Geek()
f = open("threads_testing.txt", "a")
f.write("\n-------------------")
num = input ("Enter number :")
f.write("\n threads: ")
f.write(str(num))
f.write("\ntime:")

exe_start = time.time()
dag.DAG_create()
print(dag.DAG_select())
print(dag.DAG_select())
print(dag.DAG_select())
exe_end = time.time()
cal_exe = exe_end - exe_start
f.write(str(cal_exe))

f.close()



























