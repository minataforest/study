#include <stdio.h>
#include <stdlib.h>
#include <time.h>

// 물고기 6마리가 있다. 이들은 어항에 있고 사막이다.
// 사막은 더워서 물이 빨리 증발한다.
// 물이 다 증발하기 전에 부지런히 어항에 물을 줘야 한다.
// 물고기는 시간이 지날수록 커져서 나중에는 먹는다

int level;
int arrFish[6];
int * cursor;
void initDate(); 
void printfFishes();
void decreaseWater(long elapseTime);
int checkFishAlive();

int main(void){
    long startTime = 0; // 게임 시작시간
    long totalElapsedTime = 0; // 총 경과시간
    long prevElapsedTime = 0; // 직전 경과시간(최근에 물을 준 시간 간격)

    int num; // 몇 번 어항에 물을 줄 것인지 사용자 입력
    initDate();

    cursor = arrFish; // cursor[0]...cursor[1]

    startTime = clock(); // 현재 시간을 millisecond로 반환해줌
    
    while (1)
    {
        printfFishes();
        printf("몇 번 어항에 물을 주시겠어요? ");
        scanf("%d", &num);

        //입력값 체크
        if(num < 1 || num >6){
            printf("\n입력값이 잘못되었습니다\n");
            continue;
        }

        // 총 경과시간
        totalElapsedTime = (clock() - startTime) / CLOCKS_PER_SEC; //초 단위로 변환
        printf("총 경과시간: %ld초\n", totalElapsedTime);

        // 직전에 물을 준 시간(마지막으로 물을 준 후로 흐른 시간)
        prevElapsedTime = totalElapsedTime - prevElapsedTime;
        printf("최근 경과시간: %ld초\n", prevElapsedTime);
        
        //어항의 물을 감소(증발)
        decreaseWater(prevElapsedTime);

        //사용자가 입력한 어항에 물을 준다
        // 어항에 물이 0이면 물을 주지 않는다(이미 고기는 죽었으니까)
        if(cursor[num -1] <= 0){
            printf("%d번 물고기는 이미 죽음. 물을 주지 않음\n", num);
        }
        //추가한 물 + 어항의 물이 100을 안 넘을 때만 물을 줌
        else if (cursor[num-1] + 1 <= 100){
            printf("%d번 어항에 물을 줍니다\n", num);
            cursor[num-1] += 1;
        }

        //레벨업 여부 확인(레벨업은 20초마다 수행)
        if(totalElapsedTime / 20 > level -1){
            level++;
            printf("레벨 %d에서 레벨 %d로 레벨업\n", level-1, level);

            if(level ==5){
                printf("\n최종레벨에 도달. 게임 클리어");
                exit(0);
            }
        }

        //모든 물고기가 죽었는지 확인
        if(checkFishAlive() ==0){
            printf("물고기가 모두 죽었습니다\n");
            exit(0);
        } else{
            printf("물고기가 아직 살아있습니다\n");
        }

        //임시로 총 시간을 지정해주는 거임
        prevElapsedTime = totalElapsedTime;

    }

    return 0;
}

void initDate(){
    level = 1; // 게임레벨(1-6)
    for (int i=0; i<6;i++){
        arrFish[i] = 100; //어항의 물 높이(0-100)
    }
}

void printfFishes(){
    printf("%3d번 %3d번 %3d번 %3d번 %3d번 %3d번\n", 1,2,3,4,5,6);
    for(int i=0;i<6;i++){
        printf(" %4d ", arrFish[i]);
    }
    printf("\n\n");
}

void decreaseWater(long elapseTime){
    for(int i=0; i<6;i++){
        arrFish[i] -= (level * 3 * (int)elapseTime); //3은 난이도 조절을 위한 값
        if(arrFish[i] < 0 ){
            arrFish[i] = 0;
        }
    }
}

int checkFishAlive(){
    for(int i = 0; i<6;i++){
        if(arrFish[i]>0){
            return 1;
        }
    }
    return 0;
}