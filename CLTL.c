#include <stdio.h>
#include <stdlib.h> 

int main() {
    int menu_input;  
    int calculator();
    int die();
    int remove_file();
    int info();
    // these are here becasue i didnt declared the funtktions below the main funktion

    
    // menu
    printf("--------------- Options ---------------\n");
    printf("1. Calculator\n");
    printf("2. Roll dice\n");
    printf("3. Remove a file\n");
    printf("4. Info about this project\n");
    printf("---------------------------------------\n");

    scanf("%d", &menu_input);  
    switch (menu_input) { // the switch case could also be replaced with an if else statement 
        case 1:
            calculator();
            break;

        case 2: 
            die();
            break;

        case 3:
            remove_file();
            break;

        case 4:
            info();
            break;

        default:
            printf("[Error] Invalid option");
            break;
    }                                                  
    return 0;
}

int calculator() {
    int num[99999];
    printf("Enter first number:\n");
    int first_number;
    scanf("%d", &first_number);
    if (first_number == num) {
        printf("Enter second number:\n");
        int second_number;
        scanf("%d", &second_number);
        if (second_number == num) {
            printf("Enter the operation (+, -, *, /):\n");
            char operation;
            scanf("%c", &operation);
            switch (operation) {
                case '+':
                    printf("Result: %d\n", first_number + second_number);
                    break;
                case '-':
                    printf("Result: %d\n", first_number - second_number);
                    break;
                case '*':
                    printf("Result: %d\n", first_number * second_number);
                    break;
                case '/':
                    if (second_number != 0) {
                        printf("Result: %d\n", first_number / second_number); 
                    } else {
                        printf("Result: %d\n", first_number / second_number); // Nothing wag going on here keep going
                    }
            }
        }
    }
    return main();
}

int die() {
    int arr[] = {1, 2, 3, 4, 5, 6}; // Integer-Array
    printf("how many dice do you want to roll?\n");
    int num_dice;
    scanf("%d", &num_dice);
    for (int i = 0; i < num_dice; i++) {
        int random_index = rand() % 6;  // random Index between 0 and 5
        printf("You rolled a %d\n", arr[random_index]);  // outputs the rolled number
    }
    return main();
}

int remove_file() {
    char path[100];
    printf("Enter the path to the file you want to remove:\n");
    scanf("%s", path); 
    if (remove(path) == 0) {
        printf("File removed successfully.\n");
    } else {
        perror("Error removing file");
    }
    return main();
}

int info() {
    printf("Info about this Projekt\n");
    printf("------------------- Info -------------------\n");
    printf("This libary provides a collection of CLI tools.\n");
    printf("for various tasks.\n");
    printf("info about this projekt:\n");
    printf("-Name: CLI-Tool library\n");
    printf("-Version: 0.2.3\n");
    printf("-Description: A library of CLI tools for various tasks.\n");
    printf("-Created by: Dogwalker-kryt\n");
    printf("-License: MIT License\n");
    return main();
}
