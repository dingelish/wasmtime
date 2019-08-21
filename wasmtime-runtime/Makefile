all:
	g++ -c -fpic allocate.c
	g++ -T jit.lds -shared -o libget.so allocate.o
