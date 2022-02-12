```javascript
import { createStore } from 'redux';
import { Provider, useSelector, useDispatch, connect } from 'react-redux';

function reducer(currentState, action){
  if(currentState === undefined){
    return{
      number:1,
      color:'white'
    }
  }
  const newState = {...currentState}
  if(action.type === 'PLUS'){
    newState.number++;
    newState.color = action.color
  }
  return newState
}

const store = createStore(reducer);
```

### Provider
컴포넌트. state를 제공할 컴포넌트들을 Provider 컴포넌트로 감싸주면 됨.   
props로 반드시 store를 갖고 있어야 함
```javascript
<Provider store={store}>
  <Left1></Left1>
  <Right1></Right1>
</Provider>
```

### useSelector
어떤 state 값을 쓰고 싶은지 선택한다.
```javascript
const number = useSelector((state)=> state.number);
const color = useSelector((state)=> state.color);
```

### useDispatch
state 값을 변경할 때 사용
```javascript
const dispatch = useDispatch();
onClick={()=>{dispatch({type:'PLUS', color:'blue'})}}
```

### connect
여기서는 다루지 않음. 재사용할 때 사용한다.