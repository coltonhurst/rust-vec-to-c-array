/*
    test() is used for FFI testing from Rust.

    listPeopleOnTheBus() and the types were lifted from main.c,
    this is meant to be called from Rust.

    Build Instructions
    - To build the .o file  > clang -c .\lib.c -o lib.o
    - To build the .so file > clang -shared -o .\lib.so .\lib.o
*/

#include <stdio.h>
#include <stdlib.h>

typedef struct Person {
    char *name;
    int age;
} Person;

typedef struct Bus {
    char *busName;
    Person** people;
} Bus;

int test() {
    return 5;
}

void listPeopleOnTheBus(Bus *bus, int peopleCount) {
    printf("The \"%s\" People List:\n", bus->busName);

    int i;
    struct Person *people = *(bus->people);
    for (i = 0; i < peopleCount; i++) {
        printf("  %s is on the bus, and is %d years old\n", (people + i)->name, (people + i)->age);
    }
}
