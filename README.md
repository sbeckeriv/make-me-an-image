Based off of this post

https://rogeralsing.com/2008/12/07/genetic-programming-evolution-of-mona-lisa/

Turns out I didn't really read the post. It is very little like the post
above.

1) generate random objects and add them to the buffer

2) compare fitness to base. if better promote that one else use the old
buffer

3) repeat a lot


1 million runs


![alt text](https://raw.githubusercontent.com/sbeckeriv/make-me-an-image/master/1_mill.png "1 mill")![alt text](https://raw.githubusercontent.com/sbeckeriv/make-me-an-image/master/base.png "base")

10 million runs took 2 hours 40 minutes


![alt text](https://raw.githubusercontent.com/sbeckeriv/make-me-an-image/master/run_9999999.png "10 mill")![alt text](https://raw.githubusercontent.com/sbeckeriv/make-me-an-image/master/base.png "base")


Per object fitness test 10 million 4m11.150s


![alt text](https://raw.githubusercontent.com/sbeckeriv/make-me-an-image/master/bird_10_mill.png "10 mill")![alt text](https://raw.githubusercontent.com/sbeckeriv/make-me-an-image/master/base.png "base")


1 million run of triangles old style


![alt text](https://raw.githubusercontent.com/sbeckeriv/make-me-an-image/master/run_1000000_tri.png "1 mill ")![alt text](https://raw.githubusercontent.com/sbeckeriv/make-me-an-image/master/base.png "base")


