### 1st class citizen(일급객체)
숫자: 변수의 값이 될 수 있다 -> 일급객체   
조건문: 변수의 값이 될 수 없다 -> 이급객체   
함수: 변수의 값이 될 수 없다 -> 이급객체  
  
변수의 값이 될 수 있다: 함수의 입력값, 리턴값이 될 수 있다.  


### 콜백함수
어떤 함수가 다른 함수의 입력값으로 전달되어 다른 함수에 의해서 나중에 호출되는 함수
```
val = function(1){
    return 1+1;
}

function fn(arg){
    arg();
}

fn(val); -> callback function
```
아래에서 word => word.length > 6은 콜백함수이다.  
arr.filter(callback(element[, index[, array]])[, thisArg])
```
const words = ['spray', 'limit', 'elite', 'exuberant', 'destruction', 'present'];
const result = words.filter(word => word.length > 6);
```


### promise
#### 1. promise를 사용하는 이유
비동기적인 작업을 처리할 때, 그 작업의 성공/실패 여부를 표준화된 방식으로 처리하기 위함(then/catch)

- synchronous 동기: 순차적으로 실행  
- asynchronous비동기: 병렬적(동시적)으로 실행. 복잡하지만 빠름.  

#### 2. promise 예제 (fetch)
https://developer.mozilla.org/en-US/docs/Web/API/fetch  
```
Syntax
const fetchResponsePromise = fetch(resource [, init])
// resource = url 값  

Return value
A Promise that resolves to a Response object.
```
fetch의 리턴값: promise 타입을 리턴하며, 성공 시 response 객체를 준다.

#### 3. promise의 then, catch 메소드
promise를 리턴하면, 비동기일 가능성이 높으며, promise는 then과 catch 두 개의 메소드를 사용할 수 있다.
- then:  성공한 경우 자동 호출. 결과값이 있으면 첫번째 파라미터로 받을 수 있다. 
- catch: 실패한 경우 자동 호출. 첫번째 파라미터로 실패 사유를 받을 수 있다.

#### 4. promise nesting/chaining
response.json()은 promise 객체를 반환하는 response의 함수이다.

- nesting 방식
```
fetch('https://jsonplaceholder.typicode.com/posts/1')
    .then(function(response){ 
        response.json().then(function(data){
            console.log('data', data);
        });
    })
    .catch(function(reason){ 
        console.log(reason);
    });
```
- chaining 방식
```
fetch('https://jsonplaceholder.typicode.com/posts/1')
    .then(function(response){ 
        return response.json(); // promise를 리턴한다
    })
    .catch(function(reason){ 
        console.log(reason);
    })
    .then(function(data){
        console.log('data', data);
    });
```