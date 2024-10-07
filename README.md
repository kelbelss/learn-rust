# Learn Rust 🦀

Welcome to my Rust learning repository! This is where I document my journey of learning the Rust programming language, sharing the resources, projects, and insights I've gathered along the way. Whether you're a beginner or looking to deepen your understanding of Rust, I hope you'll find this repository useful 👩🏼‍💻✨
 
## Main Resources

- [The Rust Programming Language (The Book 📖)](https://doc.rust-lang.org/book/)
- [Let's Get Rusty - The Rust Lang Book Playlist](https://youtube.com/playlist?list=PLai5B987bZ9CoVR-QEIN9foz4QCJ0H2Y8&si=ldobWHRCRxI8ha6o)
- [Rustlings](https://github.com/rust-lang/rustlings)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)

🦀 *I decided to follow the [interactive version](https://rust-book.cs.brown.edu/experiment-intro.html) of the Rust Book by Brown University*

## Videos I Recommend Watching Before You Begin 🤓

1. [How to Learn Rust](https://youtu.be/2hXNd6x9sZs?si=cTaDFsM20BbHuMzC)
2. [What Makes Rust Different?](https://youtu.be/v6RxJsk8otY?si=HeDOEj0G7hc7ilyG)
3. [5 Things I Wish I Knew Before Learning Rust](https://youtu.be/EYCBm0xAWow?si=O9MSA8RMDX9AN0Ng)
4. [Rust in 100 Seconds](https://youtu.be/5C_HPTJg5ek?si=HaMm-O_GGoCw9dsr)
5. [Stack, Heap, and Static Memory](https://youtu.be/NnLdGKoz1ls?si=YrtC5lKBaDmHcKe0)
6. [Memory Management Strategies](https://youtu.be/GUZ_2gGWuPo?si=Mc1HUagaT8tb4Ufq)
7. [Rust makes you feel like a GENIUS](https://youtu.be/0rJ94rbdteE?si=EDhFKZSyJQCNUpeH)

## Useful Videos I Keep Going Back To

1. [Rust for the impatient](https://youtu.be/br3GIIQeefY?si=CIMy5HqAmSpJ3Kgv)

## Sections (following The Book 📖)

### 1. Getting Started

- The Book 📖 - [interactive version](https://rust-book.cs.brown.edu/ch01-00-getting-started.html) or [original](https://doc.rust-lang.org/book/ch01-00-getting-started.html)
- Let's Get Rusty - [ULTIMATE Rust Lang Tutorial! - Getting Started](https://youtu.be/OX9HJsJUDxA?si=PhZ5Iv6Azi8azQf9)
- [Rustlings](https://github.com/rust-lang/rustlings) - I did the intro exercises at this point.

🦀 *If you plan to commit your progress to a single repository, I recommend adding a `.gitignore` file with `**/target/` and `**/Cargo.lock` to your root directory.*

### 2. Programming a Guessing Game

- The Book 📖 - [interactive version](https://rust-book.cs.brown.edu/ch02-00-guessing-game-tutorial.html) or [original](https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html)
- Let's Get Rusty - [Programming a Guessing Game in Rust!](https://youtu.be/H0xBSbnQYds?si=WubyeEfIZhSLnEnE) 

🦀 *The Let's Get Rusty video is great for this section. I found it very helpful to have someone explain it. I watched the video first and coded along during the second watch. He also adds color which is a fun addition.*

### 3. Common Programming Concepts

- The Book 📖 - [interactive version](https://rust-book.cs.brown.edu/ch03-00-common-programming-concepts.html) or [original](https://doc.rust-lang.org/book/ch03-00-common-programming-concepts.html)
- Let's Get Rusty - [Common Programming Concepts in Rust](https://youtu.be/2V0JaMVjzws?si=NAPRn5s_sazqWOAx) 
- Tech with Tim - [Rust Tutorial #4 - Data Types](https://youtu.be/t047Hseyj_k?si=M2P-K-_83AcgTQwW)
- [Rustlings](https://github.com/rust-lang/rustlings) - I completed the exercises in 01_variables, 02_functions, 03_if, and exercises 1, 2 and 3 in 04_primitive_types. I also completed the first quiz in the quizzes folder.
- Practice - the Rust Book suggests building a program that can generate the [*n*th Fibonacci number](https://github.com/kelbelss/learn-rust/tree/main/rust-book/common_programming_concepts/fibonacci/src). I began with a [Fibonacci in Rust](https://benjaminbrandt.com/fibonacci-in-rust/) demonstration and then worked with Claude AI to break it down and understand it better. I added a non-recursive function and a memoisation function using `std::collections::HashMap`, and then compared the time complexity of each using Rust's standard library's `std::time::Instant`. 

🦀 *For Data Types I found the Tech with Tim video helpful, along with the Let's Get Rusty video. I reccommend reading The Book too, as the videos do not cover everything.*

### 4. Understanding Ownership

- The Book 📖 - [interactive version](https://rust-book.cs.brown.edu/ch04-00-understanding-ownership.html) or [original](https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html)
- Let's Get Rusty - [Understanding Ownership in Rust](https://youtu.be/VFIOSWy93H0?si=VFcXYn3_BQx0opjt) 
- Tech with Tim - [Rust Tutorial #9 - Memory Management, Heap & Stack](https://youtu.be/-6cnnNlAvNk?si=qFCFcRIuuhJm0A0Z)
- [Ownership [25 of 35] | Rust for Beginners](https://youtu.be/2H-O43Hl94c?si=QYqk_pwGR0iCtIFg)
- [Borrowing [27 of 35] | Rust for Beginners](https://youtu.be/6eCV-Q-kjX4?si=sq5zOJsAUDM2YXs-)
- [Rustlings](https://github.com/rust-lang/rustlings) Completed 04_primitive_types 4, 5 and 6. In order to continue with Rustlings and complete 05_vecs, I watched [Common Collections in Rust](https://youtu.be/Zs-pS-egQSs?si=M0kgSO-GNg4ncuJY) and [Master Rust Collections: Vectors, Enums, and Iteration - Full Crash Rust Tutorial for Beginners](https://youtu.be/f4a8rWkSl8U?si=1HIG4tKFurR2rGqS).
- I decided to build a small project following a tutorial to get a better idea of what a Rust project looks like. I followed Akhil Sharma's [I Built A Password Vault with Rust!](https://youtu.be/YYVoOoy8d2s?si=t_pP0N9K2y1NkvlW). 

🦀 *I began this section with the Tech with Tim Tuturial 9 video to get a better understanding of memory management.*