#include <stdio.h>

int main(void){
    int arr[3] = {5, 10, 15};
    int * ptr = arr;
    for (int i=0; i < 3; i++){
        printf("배열 arr[%d]의 값: %d\n", i, arr[i]);
    }

    for (int i=0; i < 3; i++){
        printf("포인터 ptr[%d]의 값: %d\n", i, ptr[i]);
    }

    ptr[0] = 100;
    ptr[1] = 200;
    ptr[2] = 300;

     for (int i=0; i < 3; i++){
        printf("배열 arr[%d]의 값: %d\n", i, arr[i]);
        printf("배열 arr[%d]의 값: %d\n", i, *(arr+i)); // arr 주소의 i 번째 값
    }

    for (int i=0; i < 3; i++){
        printf("포인터 ptr[%d]의 값: %d\n", i, ptr[i]);
        printf("포인터 ptr[%d]의 값: %d\n", i, *(ptr+i));
    }
    // *(arr + i) = arr[i]
    // arr == arr 베열의 첫번째 값의 주소와 동일 = &arr[0]
    printf("arr 자체의 값: %d\n", arr);
    printf("arr[0]의 주소: %d\n", &arr[0]);

    printf("arr 자체의 값이 가지는 주소의 실제값: %d\n", *arr); // == *(arr +0)
    printf("arr[0]의 실제값: %d\n", *&arr[0]);
    printf("arr[0]의 실제값: %d\n", *&*&*&*&*&*&*&arr[0]);
    printf("arr[0]의 실제값: %d\n", arr[0]);

    // *& 는 아무것도 없는 것과 같다. &는 주소이며, *은 그 주소의 값이기 때문에, &과 *는 서로 상쇄된다.

    

}