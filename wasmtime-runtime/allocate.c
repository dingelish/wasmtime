#include <stdio.h>
#include <stddef.h>
extern unsigned int __jit_start;
extern unsigned int __jit_end;
size_t current_offset = 0;

/*TODO replace this toy allocator*/
unsigned char* allocate_from_jit(size_t size) {
	size_t start = (size_t)&(__jit_start);
	size_t target = current_offset + start; 
	current_offset += size;
	return (unsigned char*)target;
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
