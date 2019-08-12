all:
	gcc -c -fpic allocate.c
	gcc -T jit.lds -shared -o libget.so allocate.o
