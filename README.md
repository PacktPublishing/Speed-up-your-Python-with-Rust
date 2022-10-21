# Speed Up Your Python with Rust 

<a href="https://www.packtpub.com/product/speed-up-your-python-with-rust/9781801811446?utm_source=github&utm_medium=repository&utm_campaign="><img src="https://static.packt-cdn.com/products/9781801811446/cover/smaller" alt="Speed Up Your Python with Rust " height="256px" align="right"></a>

This is the code repository for [Speed Up Your Python with Rust ](https://www.packtpub.com/product/speed-up-your-python-with-rust/9781801811446?utm_source=github&utm_medium=repository&utm_campaign=), published by Packt.

**Optimize Python performance by creating Python pip modules in Rust with PyO3**

## What is this book about?
Python has made software development easier, but it falls short in several areas including memory management that lead to poor performance and security. Rust, on the other hand, provides memory safety without using a garbage collector, which means that with its low memory footprint, you can build high-performant and secure apps relatively easily. However, rewriting everything in Rust can be expensive and risky as there might not be package support in Rust for the problem being solved. This is where Python bindings and pip come in.

This book covers the following exciting features:
* Explore the quirks of the Rust programming language that a Python developer needs to understand to code in Rust 
* Understand the trade-offs for multiprocessing and thread safety to write concurrent code
* Build and manage a software project with cargo and crates
* Fuse Rust code with Python so that Python can import and run Rust code
* Deploy a Python Flask application in Docker that utilizes a private Rust pip module
* Inspect and create your own Python objects in Rust



If you feel this book is for you, get your [copy](https://www.amazon.com/dp/180181144X) today!

<a href="https://www.packtpub.com/?utm_source=github&utm_medium=banner&utm_campaign=GitHubBanner"><img src="https://raw.githubusercontent.com/PacktPublishing/GitHub/master/GitHub.png" 
alt="https://www.packtpub.com/" border="5" /></a>

## Instructions and Navigations
All of the code is organized into folders. For example, Chapter02.

The code will look like the following:
```
use std::error::Error;
use std::fs::File;
use csv;
use super::structs::FootPrint;
let code = "5 + 6";
let result = py.eval(code, None, Some(&locals)).unwrap();
let number = result.extract::<i32>().unwrap();
```

**Following is what you need for this book:**
This book is for Python developers who want to speed up their Python code with Rust and implement Rust in a Python system without altering the entire system. You'll be able to learn about all topics relating to Rust programming. Basic knowledge of Python is required to get the most out of this book.

With the following software and hardware list you can run all code files present in the book (Chapter 1-11).
### Software and Hardware List
| Chapter | Software required | OS required |
| -------- | ------------------------------------ | ----------------------------------- |
| 1 | Python 3 | Windows, Mac OS X, and Linux (Any) |
| 1 | Rust | Windows, Mac OS X, and Linux (Any) |
| 1 | Docker | Windows, Mac OS X, and Linux (Any) |
| 1 | PYO3 | Windows, Mac OS X, and Linux (Any) |
| 1 | Redis | Windows, Mac OS X, and Linux (Any) |
| 1 | PostgreSQL | Windows, Mac OS X, and Linux (Any) |


We also provide a PDF file that has color images of the screenshots/diagrams used in this book. [Click here to download it](https://static.packt-cdn.com/downloads/9781801811446__ColorImages.pdf).

### Related products
* Python for Geeks  [[Packt]](https://www.packtpub.com/product/python-for-geeks/9781801070119?utm_source=github&utm_medium=repository&utm_campaign=) [[Amazon]](https://www.amazon.com/dp/1801070113)

* Rust Web Programming  [[Packt]](https://www.packtpub.com/product/rust-web-programming/9781800560819?utm_source=github&utm_medium=repository&utm_campaign=) [[Amazon]](https://www.amazon.com/dp/1800560818)



## Get to Know the Author
**Maxwell Flitton**
is a software engineer who works for the open source financial loss modeling foundation OasisLMF. In 2011, Maxwell achieved his Bachelor of Science degree in nursing from the University of Lincoln, UK. While working 12-hour shift s in the A&E departments of hospitals, Maxwell obtained another degree in physics from the Open University in the UK and then moved on to another milestone, with a postgraduate diploma in physics and engineering in medicine from UCL in London. He's worked on numerous projects such as medical simulation soft ware for the German government and supervising computational medicine students at Imperial College London. He also has experience in financial tech and Monolith AI.



## Other books by the authors
[Rust Web Programming ](https://www.packtpub.com/product/rust-web-programming/9781800560819?utm_source=github&utm_medium=repository&utm_campaign=)



### Download a free PDF

 <i>If you have already purchased a print or Kindle version of this book, you can get a DRM-free PDF version at no cost.<br>Simply click on the link to claim your free PDF.</i>
<p align="center"> <a href="https://packt.link/free-ebook/9781801811446">https://packt.link/free-ebook/9781801811446 </a> </p>