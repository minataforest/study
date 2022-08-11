#include <stdio.h>
void swap(int a, int b);
void swap_addr(int * a, int * b);

int main(void){
    int a = 10;
    int b = 20;
    printf("a의 주소: %d\n", &a);
    printf("b의 주소: %d\n", &b);

    //a와 b의 값을 바꾼다
    //값에 의한 복사 (call by value) -> 값만 복사한다는 의미
    printf("Swap 함수 전 => a: %d, b: %d\n", a, b);
    swap(a, b);
    printf("Swap 함수 후 => a: %d, b: %d\n", a, b);

    //메모리 공간에 있는 주소값 자체를 넘긴다.
    printf("==주소값 전달==\n");
    printf("Swap_addr 함수 전 => a: %d, b: %d\n", a, b);
    swap_addr(&a, &b);
    printf("Swap_addr 함수 후 => a: %d, b: %d\n", a, b);
    return 0;
}

void swap(int a, int b){
    printf("swap 함수 내 a의 주소: %d\n", &a);
    printf("swap 함수 내 b의 주소: %d\n", &b);
    int temp = a;
    a = b;
    b = temp;
    printf("Swap 내 => a: %d, b: %d\n", a, b);
}

void swap_addr(int * a, int * b){
    int temp = *a;
    *a = *b;
    *b = temp;
    printf("Swap_addr 내 => a: %d, b: %d\n", *a, *b);
}