```
$ sudo apt install gnuplot
$ gnuplot alloc.plot
```

X-axis : alloc size

Y-axis : alloc time

힙 할당 속도를 할당 크기에 대비해 찍어 보면 둘 간에 명확한 관계가 없을을 할 수 있다.

동일한 양의 메모리를 여러 번 요청한다 할지라도 메모리 할당에 드는 시간은 근본적으로 예측 불가능하다.
