// this is an example contribution in other languages than Rust.

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>

typedef struct {
    char* question;
    char* correct_answer;
} QuestionAnswer;

QuestionAnswer generate_arithmetic_question();

int main() {
    srand(time(NULL));

    QuestionAnswer qa = generate_arithmetic_question();

    if (qa.question != NULL && qa.correct_answer != NULL) {
        printf("%s\n", qa.question);
        printf("Correct Answer: %s\n", qa.correct_answer);
        free(qa.question);
        free(qa.correct_answer);
    }

    return 0;
}

QuestionAnswer generate_arithmetic_question() {
    int num1 = rand() % 100 + 1;
    int num2 = rand() % 100 + 1;
    char operations[] = {'+', '-', '*', '/'};
    int opIndex = rand() % 4;
    int correct_answer_numeric;
  
    switch (operations[opIndex]) {
        case '+':
            correct_answer_numeric = num1 + num2;
            break;
        case '-':
            correct_answer_numeric = num1 - num2;
            break;
        case '*':
            correct_answer_numeric = num1 * num2;
            break;
        case '/':
            while (num2 == 0) {
                num2 = rand() % 100 + 1;
            }
            correct_answer_numeric = num1 / num2;
            break;
    }

    char* question = (char*)malloc(50 * sizeof(char));
    char* correct_answer = (char*)malloc(20 * sizeof(char));

    if (question == NULL || correct_answer == NULL) {
        printf("Memory allocation failed\n");
        if (question) free(question);
        if (correct_answer) free(correct_answer);
        return (QuestionAnswer){NULL, NULL};
    }

    return (QuestionAnswer){question, correct_answer};
}
