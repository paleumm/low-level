#include <sys/types.h>
#include <sys/stat.h>
#include <fcntl.h>
#include <stdio.h>

int main() {
    int fd;
    fd = open("/nothing", O_RDONLY);
    
    if (fd == -1)
        printf("Hell Yeah!\n");
}