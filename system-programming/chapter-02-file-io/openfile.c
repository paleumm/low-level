#include <sys/types.h>
#include <sys/stat.h>
#include <fcntl.h>
#include <stdio.h>

int main() {
    int fd;
    char *file = "./nothing";
    fd = open(file, O_RDONLY); // error for sure
    if (fd == -1)
        printf("File: \"%9s\" not found\n", file);
    
    int f;
    f = open("./test.txt", O_WRONLY | O_CREAT | O_TRUNC, S_IWUSR | S_IRUSR | S_IWGRP | S_IRGRP | S_IROTH);
    if (f == -1) printf("NONO\n");
}