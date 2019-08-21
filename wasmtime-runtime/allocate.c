#include <stdio.h>
#include <stddef.h>
#include <mutex>
extern unsigned int __jit_start;
extern unsigned int __jit_end;
std::mutex g_mutex;
size_t current_offset = 0;
/*TODO replace this toy allocator*/
extern "C" {
	unsigned char* allocate_from_jit(size_t size) {
		size_t start = (size_t)&(__jit_start);
		start = start >> 12;
		start = start << 12;
		start += 4096;
		g_mutex.lock();
		size_t target = current_offset + start;
		current_offset += size;
		//printf("allocate offset is now %lu\n",current_offset);
		if (current_offset > (size_t)&(__jit_end)) {
			return 0;
		}
		g_mutex.unlock();
		for(size_t clean=0;clean<size;clean++)
			*(unsigned char*)(target + clean) = 0x00;
		//	for(size_t clean=0;clean<128*256*4096;clean++)
		//		*(unsigned char*)(start + clean) = 0x00;
		return (unsigned char*)target;
	}

	void init() {
		printf("init is called\n");
		size_t start = (size_t)&(__jit_start);
		for(size_t clean=0;clean<128*256*4096;clean++)
			*(unsigned char*)(start + clean) = 0x00;
	}
	/*TODO implement free*/

	unsigned char* get_address_start()
	{
		return (unsigned char*)&(__jit_start);
	}

	unsigned char* get_address_end()
	{
		return (unsigned char*)&(__jit_end);
	}

	void free_jit_memory() {
		//  printf("free jit memory call\n");
	}
}
