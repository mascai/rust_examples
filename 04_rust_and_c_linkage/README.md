# How to run

```
gcc -fPIC -shared -o libfoo.so foo.c # create dynamic link library (DLL) with name libfoo.so 
rustc -l foo -L . main.rs # link DLL with name libfoo.so  with main.rs binary
env LD_LIBRARY_PATH="$LD_LIBRARY_PATH:." ./main # run main binary and specify path to find DLL
```
