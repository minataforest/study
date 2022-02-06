import './App.css';
import React from 'react';
import { createStore } from 'redux';
import { Provider, useSelector, useDispatch, connect } from 'react-redux';
// Provider: 컴포넌트. state들을 제공힐 컴포넌트들을 <Provider> 컴포넌트로 감싸주면 됨.
//            props로 반드시 store를 갖고 있어야 함
// useSelector: 어떤 state 값을 쓰고 싶은지 선택.
//            const number = useSelector((state)=> state.number);
// useDispatch: state 값 변경할 때 사용
//            const dispatch = useDispatch();
//            onClick={()=>{dispatch({type:'PLUS'})}}
// connect. 사용하기 어려워서 안 씀 재사용할 때만 사용

function reducer(currentState, action){
  if(currentState === undefined){
    return{
      number:1
    }
  }
  const newState = {...currentState}
  if(action.type === 'PLUS'){
    newState.number++;
  }
  return newState
}

const store = createStore(reducer);

export default function App() {

  return (
    <div id="container">
      <h1>Root</h1>
      <div id="grid">
        <Provider store={store}>
          <Left1></Left1>
          <Right1></Right1>
        </Provider>
      </div>
    </div>
  );
}

function Left1(props){
  return(
    <div>
      <h1>Left1</h1>
      <Left2></Left2>
    </div>
  )
}

function Left2(props){
  console.log(2);
  return(
    <div>
      <h1>Left2: </h1>
      <Left3></Left3>
    </div>
  )
}

function Left3(props){
  console.log(3);
  // function f(state){
  //   return state.number;
  // }
  // const number = useSelector(f);
  const number = useSelector((state)=> state.number);
  return(
    <div>
      <h1>Left3: {number}</h1>
    </div>
  )
}

function Right1(props){
  return(
    <div>
      <h1>Right1</h1>
      <Right2></Right2>
    </div>
  )
}

function Right2(props){
  return(
    <div>
      <h1>Right2</h1>
      <Right3></Right3>
    </div>
  )
}

function Right3(props){
  const dispatch = useDispatch();
  return(
    <div>
      <h1>Right3</h1>
      <input type="button" value="+" onClick={()=>{
        dispatch({type:'PLUS'})
      }}
      ></input>
    </div>
  )
}
