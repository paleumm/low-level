#include <sys/types.h>
#include <sys/stat.h>
#include <fcntl.h>
#include <stdio.h>

int main() {
    char *file = "./createfile";
    int fd = creat(file, 0644); // same as fd = open (filename, O_WRONLY | O_CREAT | O_TRUNC, 0644);
    if (fd == -1) printf("Oops!\n");
    return 0;
}