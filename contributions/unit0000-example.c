// this is an example contribution in other languages than Rust.

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>

char* generate_arithmetic_question(int* correct_answer);

int main() {
    srand(time(NULL));

    int answer;
    char* question = generate_arithmetic_question(&answer);

    if (question != NULL) {
        printf("%s\n", question);
        printf("Correct Answer: %d\n", answer);
        free(question);
    }

    return 0;
}

char* generate_arithmetic_question(int* correct_answer) {
    int num1 = rand() % 100 + 1;
    int num2 = rand() % 100 + 1;
    char operations[] = {'+', '-', '*', '/'};
    int opIndex = rand() % 4;
  
    switch (operations[opIndex]) {
        case '+':
            *correct_answer = num1 + num2;
            break;
        case '-':
            *correct_answer = num1 - num2;
            break;
        case '*':
            *correct_answer = num1 * num2;
            break;
        case '/':
            while (num2 == 0) {
                num2 = rand() % 100 + 1;
            }
            *correct_answer = num1 / num2;
            break;
    }

    char* question = (char*)malloc(50 * sizeof(char));
    if (question == NULL) {
        printf("Memory allocation failed\n");
        return NULL;
    }

    // Format the question string
    snprintf(question, 50, "What is %d %c %d?", num1, operations[opIndex], num2);

    return question;
}
