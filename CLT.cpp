#include <iostream>
#include <string>
#include <cstdlib>

void calculator() {
    std::cout << "----- Calculator -----\n";
    std::cout << "Enter the first number: ";
    double num1;
    std::cin >> num1;
    int num0 = 9999;
    if (num1 < num0) {
        std:: cout << "Enter the Oprerator (+, -, *, /): \n";
        char op;
        std::cin >> op;
        switch (op) {
            case '+': {
                std::cout << "Enter the second number:\n";
                double num2;
                std::cin >> num2;
                if (num2 < num0) {
                    std::cout << "Calculation: " << num1 << " + " << num2 << " = " << (num1 + num2) << "\n";
                    return;
                } else {
                    std::cout << "Invalid second number!\n";
                    return;
                }
            }
            case '-': {
                std::cout << "Enther the second number:\n";
                double num2;
                std::cin >> num2;
                if (num2 < num0) {
                    std::cout << "Calculation: " << num1 << " - " << num2 << " = " << (num1 - num2) << "\n";
                    return;
                } else {
                    std::cout << "Invalid second number!\n";
                    return;
                }
            }
            case '*': {
                std::cout << "Enter the second number:\n";
                double num2;
                std::cin >> num2;
                if (num2 < num0) {
                    std::cout << "Calculation: " << num1 << " * " << num2 << " = " << (num1 * num2) << "\n";
                    return;
                } else {
                    std::cout << "Invalid second number!\n";
                    return;
                }
            }
            case '/': {
                std::cout << "Enter the second number:\n";
                double num2;
                std::cin >> num2;
                if (num2 < num0) {
                    if (num2 != 0) {
                        std::cout << "Calculation: " << num1 << " / " << num2 << " = " << (num1 / num2) << "\n";
                    } else {
                        std::cout << "Division by zero error!\n";
                    }
                    return;
                } else {
                    std::cout << "Invalid second number!\n";
                    return;
                }
            }
        }
    } else {
        std::cout << "Invalid first number!\n";
        return;
    }
    return;
}


void dice() {
    std::cout << "------ Dice Roller ------\n";
    std::cout << "Enter the number of dice to roll:\n";
    int numDice;
    std::cin >> numDice;
    if (numDice <= 0) {
        std::cout << "Invalid number of dice!\n";
        return;
    } else {
        std::cout << "Rolling " << numDice << " dice...\n";
        for (int i = 0; i < numDice; i++) {
            int roll = rand() % 6 + 1; 
            std::cout << "Dice " << (i + 1) << " rolled: " << roll << "\n";   
        }
    }
    return;
}

void removefile() {
    std::cout << "----- Remove File -----\n";
    std::cout << "Enter the file path to remove:\n";
    std::string filepath;
    std::cin >> filepath;
    if (remove(filepath.c_str()) == 0) {
        std::cout << "File removed seccessfully\n";
    } else {
        std::cout << "Error removing file: " << filepath << "\n";
    }
    return;
}


int main() {
    std::cout << "Welcome to Cli Tools!\n";
    std::cout << "----------------- menu -----------------\n";
    std::cout << "1. Calculator\n";
    std::cout << "2. dice\n";
    std::cout << "3. remove file\n";
    std::cout << "4. Exit\n";
    std::cout << "----------------------------------------\n";
    int choice;
    std::cout << "Enter your choice:\n";
    std::cin >> choice;
    switch (choice) {
        case 1:
            calculator();
            break;
        case 2:
            dice();
            break;
        case 3:
            removefile();
            break;
        case 4:
            std::cout << "Exiting...\n";
            return 0;
        default:
            std::cout << "Invalid choice. Please try again.\n";
            return 1;
    }
    return 0;
}
