/*
    This program demonstrates two custom types, a Person and a Bus.
    The Bus contains a pointer to a list of people pointers stored
    in memory.

    OUTPUT:

    > gcc main.c && ./a.out
    Bobby is 100 years old
    Susan is 99 years old
    `people` check: Bobby is 100 years old

    The "city bus" People List:
    Bobby is on the bus, and is 100 years old
    Susan is on the bus, and is 99 years old
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

void listPeopleOnTheBus(Bus *bus, int peopleCount) {
    printf("The \"%s\" People List:\n", bus->busName);

    int i;
    struct Person *people = *(bus->people);
    for (i = 0; i < peopleCount; i++) {
        printf("  %s is on the bus, and is %d years old\n", (people + i)->name, (people + i)->age);
    }
}

int main() {
    // Bobby
    char *bName = "Bobby";
    Person bob;
    bob.name = bName;
    bob.age = 100;

    // Susan
    char *sName = "Susan";
    Person susan;
    susan.name = sName;
    susan.age = 99;

    // Print info about Bobby & Susan
    printf("%s is %d years old\n", bob.name, bob.age);
    printf("%s is %d years old\n", susan.name, susan.age);

    // Add Bobby & Susan to `people`
    struct Person *people;
    people = (struct Person*) malloc(2 * sizeof(struct Person));
    people->name = bob.name;
    people->age = bob.age;
    (people + 1)->name = susan.name;
    (people + 1)->age = susan.age;

    // Check `people`
    printf("`people` check: %s is %d years old\n\n", people->name, people->age);

    // Bus
    char *busName = "city bus";
    Bus bus;
    bus.busName = busName;
    bus.people = &people;

    // Print out the people who are on the bus
    listPeopleOnTheBus(&bus, 2);

    // Free `people`
    free(people);

    return 0;
}
