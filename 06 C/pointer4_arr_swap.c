#include <stdio.h>
void changeArr(int * ptr);

int main(void){
    int arr[3] = {10, 20, 30};
    changeArr(&arr[0]);
    // changeArr(arr);
    for(int i=0; i<3;i++){
        printf("%d\n", arr[i]);
    }

    return 0;
}

void changeArr(int * ptr){
    ptr[2] = 50;
}